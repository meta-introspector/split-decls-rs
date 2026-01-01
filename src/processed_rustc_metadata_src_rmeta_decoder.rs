/* FP:decoder.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_rmeta_decoder_UNPARSEABLE_0001
/* FP:decoder.rs-0002 */ // Decoding metadata from a single crate's metadata
/* FP:decoder.rs-0003 */ 
/* FP:decoder.rs-0004 */ use std::iter::TrustedLen;
/* FP:decoder.rs-0005 */ use std::path::{Path, PathBuf};
/* FP:decoder.rs-0006 */ use std::sync::{Arc, OnceLock};
/* FP:decoder.rs-0007 */ use std::{io, mem};
/* FP:decoder.rs-0008 */ 
/* FP:decoder.rs-0009 */ pub(super) use cstore_impl::provide;
/* FP:decoder.rs-0010 */ use rustc_ast as ast;
/* FP:decoder.rs-0011 */ use crate::rustc_data_structures::fingerprint::Fingerprint;
/* FP:decoder.rs-0012 */ use crate::rustc_data_structures::fx::FxIndexMap;
/* FP:decoder.rs-0013 */ use crate::rustc_data_structures::owned_slice::OwnedSlice;
/* FP:decoder.rs-0014 */ use crate::rustc_data_structures::sync::Lock;
/* FP:decoder.rs-0015 */ use crate::rustc_data_structures::unhash::UnhashMap;
/* FP:decoder.rs-0016 */ use crate::rustc_expand::base::{SyntaxExtension, SyntaxExtensionKind};
/* FP:decoder.rs-0017 */ use crate::rustc_expand::proc_macro::{AttrProcMacro, BangProcMacro, DeriveProcMacro};
/* FP:decoder.rs-0018 */ use crate::rustc_complete::Safety;
/* FP:decoder.rs-0019 */ use crate::rustc_complete::def::Res;
/* FP:decoder.rs-0020 */ use crate::rustc_complete::def_id::{CRATE_DEF_INDEX, LOCAL_CRATE};
/* FP:decoder.rs-0021 */ use crate::rustc_complete::definitions::{DefPath, DefPathData};
/* FP:decoder.rs-0022 */ use crate::rustc_complete::diagnostic_items::DiagnosticItems;
/* FP:decoder.rs-0023 */ use crate::rustc_index::Idx;
/* FP:decoder.rs-0024 */ use crate::rustc_complete::middle::lib_features::LibFeatures;
/* FP:decoder.rs-0025 */ use crate::rustc_complete::mir::interpret::{AllocDecodingSession, AllocDecodingState};
/* FP:decoder.rs-0026 */ use crate::rustc_complete::ty::Visibility;
/* FP:decoder.rs-0027 */ use crate::rustc_complete::ty::codec::TyDecoder;
/* FP:decoder.rs-0028 */ use crate::rustc_complete::{bug, implement_ty_decoder};
/* FP:decoder.rs-0029 */ use crate::rustc_proc_macro::bridge::client::ProcMacro;
/* FP:decoder.rs-0030 */ use crate::rustc_serialize::opaque::MemDecoder;
/* FP:decoder.rs-0031 */ use crate::rustc_serialize::{Decodable, Decoder};
/* FP:decoder.rs-0032 */ use crate::rustc_complete::Session;
/* FP:decoder.rs-0033 */ use crate::rustc_complete::config::TargetModifier;
/* FP:decoder.rs-0034 */ use crate::rustc_complete::cstore::{CrateSource, ExternCrate};
/* FP:decoder.rs-0035 */ use crate::rustc_complete::hygiene::HygieneDecodeContext;
/* FP:decoder.rs-0036 */ use crate::rustc_complete::{
/* FP:decoder.rs-0037 */     BytePos, ByteSymbol, DUMMY_SP, Pos, SpanData, SpanDecoder, Symbol, SyntaxContext, kw,
/* FP:decoder.rs-0038 */ };
/* FP:decoder.rs-0039 */ use tracing::debug;
/* FP:decoder.rs-0040 */ 
/* FP:decoder.rs-0041 */ use crate::creader::CStore;
/* FP:decoder.rs-0042 */ use crate::rmeta::table::IsDefault;
/* FP:decoder.rs-0043 */ use crate::rmeta::*;
/* FP:decoder.rs-0044 */ 
/* FP:decoder.rs-0046 */ 
/* FP:decoder.rs-0047 */ /// A reference to the raw binary version of crate metadata.
/* FP:decoder.rs-0048 */ /// This struct applies [`MemDecoder`]'s validation when constructed
/* FP:decoder.rs-0049 */ /// so that later constructions are guaranteed to succeed.
/* FP:decoder.rs-0050 */ pub(crate) struct MetadataBlob(OwnedSlice);
/* FP:decoder.rs-0051 */ 
/* FP:decoder.rs-0052 */ impl std::ops::Deref for MetadataBlob {
/* FP:decoder.rs-0053 */     type Target = [u8];
/* FP:decoder.rs-0054 */ 
/* FP:decoder.rs-0055 */     #[inline]
/* FP:decoder.rs-0056 */     fn deref(&self) -> &[u8] {
/* FP:decoder.rs-0057 */         &self.0[..]
/* FP:decoder.rs-0058 */     }
/* FP:decoder.rs-0059 */ }
/* FP:decoder.rs-0060 */ 
/* FP:decoder.rs-0061 */ impl MetadataBlob {
/* FP:decoder.rs-0062 */     /// Runs the [`MemDecoder`] validation and if it passes, constructs a new [`MetadataBlob`].
/* FP:decoder.rs-0063 */     pub(crate) fn new(slice: OwnedSlice) -> Result<Self, ()> {
/* FP:decoder.rs-0064 */         if MemDecoder::new(&slice, 0).is_ok() { Ok(Self(slice)) } else { Err(()) }
/* FP:decoder.rs-0065 */     }
/* FP:decoder.rs-0066 */ 
/* FP:decoder.rs-0067 */     /// Since this has passed the validation of [`MetadataBlob::new`], this returns bytes which are
/* FP:decoder.rs-0068 */     /// known to pass the [`MemDecoder`] validation.
/* FP:decoder.rs-0069 */     pub(crate) fn bytes(&self) -> &OwnedSlice {
/* FP:decoder.rs-0070 */         &self.0
/* FP:decoder.rs-0071 */     }
/* FP:decoder.rs-0072 */ }
/* FP:decoder.rs-0073 */ 
/* FP:decoder.rs-0074 */ /// A map from external crate numbers (as decoded from some crate file) to
/* FP:decoder.rs-0075 */ /// local crate numbers (as generated during this session). Each external
/* FP:decoder.rs-0076 */ /// crate may refer to types in other external crates, and each has their
/* FP:decoder.rs-0077 */ /// own crate numbers.
/* FP:decoder.rs-0078 */ pub(crate) type CrateNumMap = IndexVec<CrateNum, CrateNum>;
/* FP:decoder.rs-0079 */ 
/* FP:decoder.rs-0080 */ /// Target modifiers - abi or exploit mitigations flags
/* FP:decoder.rs-0081 */ pub(crate) type TargetModifiers = Vec<TargetModifier>;
/* FP:decoder.rs-0082 */ 
/* FP:decoder.rs-0083 */ pub(crate) struct CrateMetadata {
/* FP:decoder.rs-0084 */     /// The primary crate data - binary metadata blob.
/* FP:decoder.rs-0085 */     blob: MetadataBlob,
/* FP:decoder.rs-0086 */ 
/* FP:decoder.rs-0087 */     // --- Some data pre-decoded from the metadata blob, usually for performance ---
/* FP:decoder.rs-0088 */     /// Data about the top-level items in a crate, as well as various crate-level metadata.
/* FP:decoder.rs-0089 */     root: CrateRoot,
/* FP:decoder.rs-0090 */     /// Trait impl data.
/* FP:decoder.rs-0091 */     /// FIXME: Used only from queries and can use query cache,
/* FP:decoder.rs-0092 */     /// so pre-decoding can probably be avoided.
/* FP:decoder.rs-0093 */     trait_impls: FxIndexMap<(u32, DefIndex), LazyArray<(DefIndex, Option<SimplifiedType>)>>,
/* FP:decoder.rs-0094 */     /// Inherent impls which do not follow the normal coherence rules.
/* FP:decoder.rs-0095 */     ///
/* FP:decoder.rs-0096 */     /// These can be introduced using either `#[rustc_coherence_is_core]`
/* FP:decoder.rs-0097 */     /// or `#[rustc_allow_incoherent_impl]`.
/* FP:decoder.rs-0098 */     incoherent_impls: FxIndexMap<SimplifiedType, LazyArray<DefIndex>>,
/* FP:decoder.rs-0099 */     /// Proc macro descriptions for this crate, if it's a proc macro crate.
/* FP:decoder.rs-0100 */     raw_proc_macros: Option<&'static [ProcMacro]>,
/* FP:decoder.rs-0101 */     /// Source maps for code from the crate.
/* FP:decoder.rs-0102 */     source_map_import_info: Lock<Vec<Option<ImportedSourceFile>>>,
/* FP:decoder.rs-0103 */     /// For every definition in this crate, maps its `DefPathHash` to its `DefIndex`.
/* FP:decoder.rs-0104 */     def_path_hash_map: DefPathHashMapRef<'static>,
/* FP:decoder.rs-0105 */     /// Likewise for ExpnHash.
/* FP:decoder.rs-0106 */     expn_hash_map: OnceLock<UnhashMap<ExpnHash, ExpnIndex>>,
/* FP:decoder.rs-0107 */     /// Used for decoding interpret::AllocIds in a cached & thread-safe manner.
/* FP:decoder.rs-0108 */     alloc_decoding_state: AllocDecodingState,
/* FP:decoder.rs-0109 */     /// Caches decoded `DefKey`s.
/* FP:decoder.rs-0110 */     def_key_cache: Lock<FxHashMap<DefIndex, DefKey>>,
/* FP:decoder.rs-0111 */ 
/* FP:decoder.rs-0112 */     // --- Other significant crate properties ---
/* FP:decoder.rs-0113 */     /// ID of this crate, from the current compilation session's point of view.
/* FP:decoder.rs-0114 */     cnum: CrateNum,
/* FP:decoder.rs-0115 */     /// Maps crate IDs as they are were seen from this crate's compilation sessions into
/* FP:decoder.rs-0116 */     /// IDs as they are seen from the current compilation session.
/* FP:decoder.rs-0117 */     cnum_map: CrateNumMap,
/* FP:decoder.rs-0118 */     /// Same ID set as `cnum_map` plus maybe some injected crates like panic runtime.
/* FP:decoder.rs-0119 */     dependencies: Vec<CrateNum>,
/* FP:decoder.rs-0120 */     /// How to link (or not link) this crate to the currently compiled crate.
/* FP:decoder.rs-0121 */     dep_kind: CrateDepKind,
/* FP:decoder.rs-0122 */     /// Filesystem location of this crate.
/* FP:decoder.rs-0123 */     source: Arc<CrateSource>,
/* FP:decoder.rs-0124 */     /// Whether or not this crate should be consider a private dependency.
/* FP:decoder.rs-0125 */     /// Used by the 'exported_private_dependencies' lint, and for determining
/* FP:decoder.rs-0126 */     /// whether to emit suggestions that reference this crate.
/* FP:decoder.rs-0127 */     private_dep: bool,
/* FP:decoder.rs-0128 */     /// The hash for the host proc macro. Used to support `-Z dual-proc-macro`.
/* FP:decoder.rs-0129 */     host_hash: Option<Svh>,
/* FP:decoder.rs-0130 */     /// The crate was used non-speculatively.
/* FP:decoder.rs-0131 */     used: bool,
/* FP:decoder.rs-0132 */ 
/* FP:decoder.rs-0133 */     /// Additional data used for decoding `HygieneData` (e.g. `SyntaxContext`
/* FP:decoder.rs-0134 */     /// and `ExpnId`).
/* FP:decoder.rs-0135 */     /// Note that we store a `HygieneDecodeContext` for each `CrateMetadata`. This is
/* FP:decoder.rs-0136 */     /// because `SyntaxContext` ids are not globally unique, so we need
/* FP:decoder.rs-0137 */     /// to track which ids we've decoded on a per-crate basis.
/* FP:decoder.rs-0138 */     hygiene_context: HygieneDecodeContext,
/* FP:decoder.rs-0139 */ 
/* FP:decoder.rs-0140 */     // --- Data used only for improving diagnostics ---
/* FP:decoder.rs-0141 */     /// Information about the `extern crate` item or path that caused this crate to be loaded.
/* FP:decoder.rs-0142 */     /// If this is `None`, then the crate was injected (e.g., by the allocator).
/* FP:decoder.rs-0143 */     extern_crate: Option<ExternCrate>,
/* FP:decoder.rs-0144 */ }
/* FP:decoder.rs-0145 */ 
/* FP:decoder.rs-0146 */ /// Holds information about a crate::rustc_span::SourceFile imported from another crate.
/* FP:decoder.rs-0147 */ /// See `imported_source_file()` for more information.
/* FP:decoder.rs-0148 */ #[derive(Clone)]
/* FP:decoder.rs-0149 */ struct ImportedSourceFile {
/* FP:decoder.rs-0150 */     /// This SourceFile's byte-offset within the source_map of its original crate
/* FP:decoder.rs-0151 */     original_start_pos: crate::rustc_span::BytePos,
/* FP:decoder.rs-0152 */     /// The end of this SourceFile within the source_map of its original crate
/* FP:decoder.rs-0153 */     original_end_pos: crate::rustc_span::BytePos,
/* FP:decoder.rs-0154 */     /// The imported SourceFile's representation within the local source_map
/* FP:decoder.rs-0155 */     translated_source_file: Arc<crate::rustc_span::SourceFile>,
/* FP:decoder.rs-0156 */ }
/* FP:decoder.rs-0157 */ 
/* FP:decoder.rs-0158 */ pub(super) struct DecodeContext<'a, 'tcx> {
/* FP:decoder.rs-0159 */     opaque: MemDecoder<'a>,
/* FP:decoder.rs-0160 */     cdata: Option<CrateMetadataRef<'a>>,
/* FP:decoder.rs-0161 */     blob: &'a MetadataBlob,
/* FP:decoder.rs-0162 */     sess: Option<&'tcx Session>,
/* FP:decoder.rs-0163 */     tcx: Option<TyCtxt<'tcx>>,
/* FP:decoder.rs-0164 */ 
/* FP:decoder.rs-0165 */     lazy_state: LazyState,
/* FP:decoder.rs-0166 */ 
/* FP:decoder.rs-0167 */     // Used for decoding interpret::AllocIds in a cached & thread-safe manner.
/* FP:decoder.rs-0168 */     alloc_decoding_session: Option<AllocDecodingSession<'a>>,
/* FP:decoder.rs-0169 */ }
/* FP:decoder.rs-0170 */ 
/* FP:decoder.rs-0171 */ /// Abstract over the various ways one can create metadata decoders.
/* FP:decoder.rs-0172 */ pub(super) trait Metadata<'a, 'tcx>: Copy {
/* FP:decoder.rs-0173 */     fn blob(self) -> &'a MetadataBlob;
/* FP:decoder.rs-0174 */ 
/* FP:decoder.rs-0175 */     fn cdata(self) -> Option<CrateMetadataRef<'a>> {
/* FP:decoder.rs-0176 */         None
/* FP:decoder.rs-0177 */     }
/* FP:decoder.rs-0178 */     fn sess(self) -> Option<&'tcx Session> {
/* FP:decoder.rs-0179 */         None
/* FP:decoder.rs-0180 */     }
/* FP:decoder.rs-0181 */     fn tcx(self) -> Option<TyCtxt<'tcx>> {
/* FP:decoder.rs-0182 */         None
/* FP:decoder.rs-0183 */     }
/* FP:decoder.rs-0184 */ 
/* FP:decoder.rs-0185 */     fn decoder(self, pos: usize) -> DecodeContext<'a, 'tcx> {
/* FP:decoder.rs-0186 */         let tcx = self.tcx();
/* FP:decoder.rs-0187 */         DecodeContext {
/* FP:decoder.rs-0188 */             // FIXME: This unwrap should never panic because we check that it won't when creating
/* FP:decoder.rs-0189 */             // `MetadataBlob`. Ideally we'd just have a `MetadataDecoder` and hand out subslices of
/* FP:decoder.rs-0190 */             // it as we do elsewhere in the compiler using `MetadataDecoder::split_at`. But we own
/* FP:decoder.rs-0191 */             // the data for the decoder so holding onto the `MemDecoder` too would make us a
/* FP:decoder.rs-0192 */             // self-referential struct which is downright goofy because `MetadataBlob` is already
/* FP:decoder.rs-0193 */             // self-referential. Probably `MemDecoder` should contain an `OwnedSlice`, but that
/* FP:decoder.rs-0194 */             // demands a significant refactoring due to our crate graph.
/* FP:decoder.rs-0195 */             opaque: MemDecoder::new(self.blob(), pos).unwrap(),
/* FP:decoder.rs-0196 */             cdata: self.cdata(),
/* FP:decoder.rs-0197 */             blob: self.blob(),
/* FP:decoder.rs-0198 */             sess: self.sess().or(tcx.map(|tcx| tcx.sess)),
/* FP:decoder.rs-0199 */             tcx,
/* FP:decoder.rs-0200 */             lazy_state: LazyState::NoNode,
/* FP:decoder.rs-0201 */             alloc_decoding_session: self
/* FP:decoder.rs-0202 */                 .cdata()
/* FP:decoder.rs-0203 */                 .map(|cdata| cdata.cdata.alloc_decoding_state.new_decoding_session()),
/* FP:decoder.rs-0204 */         }
/* FP:decoder.rs-0205 */     }
/* FP:decoder.rs-0206 */ }
/* FP:decoder.rs-0207 */ 
/* FP:decoder.rs-0208 */ impl<'a, 'tcx> Metadata<'a, 'tcx> for &'a MetadataBlob {
/* FP:decoder.rs-0209 */     #[inline]
/* FP:decoder.rs-0210 */     fn blob(self) -> &'a MetadataBlob {
/* FP:decoder.rs-0211 */         self
/* FP:decoder.rs-0212 */     }
/* FP:decoder.rs-0213 */ }
/* FP:decoder.rs-0214 */ 
/* FP:decoder.rs-0215 */ impl<'a, 'tcx> Metadata<'a, 'tcx> for (&'a MetadataBlob, &'tcx Session) {
/* FP:decoder.rs-0216 */     #[inline]
/* FP:decoder.rs-0217 */     fn blob(self) -> &'a MetadataBlob {
/* FP:decoder.rs-0218 */         self.0
/* FP:decoder.rs-0219 */     }
/* FP:decoder.rs-0220 */ 
/* FP:decoder.rs-0221 */     #[inline]
/* FP:decoder.rs-0222 */     fn sess(self) -> Option<&'tcx Session> {
/* FP:decoder.rs-0223 */         let (_, sess) = self;
/* FP:decoder.rs-0224 */         Some(sess)
/* FP:decoder.rs-0225 */     }
/* FP:decoder.rs-0226 */ }
/* FP:decoder.rs-0227 */ 
/* FP:decoder.rs-0228 */ impl<'a, 'tcx> Metadata<'a, 'tcx> for CrateMetadataRef<'a> {
/* FP:decoder.rs-0229 */     #[inline]
/* FP:decoder.rs-0230 */     fn blob(self) -> &'a MetadataBlob {
/* FP:decoder.rs-0231 */         &self.cdata.blob
/* FP:decoder.rs-0232 */     }
/* FP:decoder.rs-0233 */     #[inline]
/* FP:decoder.rs-0234 */     fn cdata(self) -> Option<CrateMetadataRef<'a>> {
/* FP:decoder.rs-0235 */         Some(self)
/* FP:decoder.rs-0236 */     }
/* FP:decoder.rs-0237 */ }
/* FP:decoder.rs-0238 */ 
/* FP:decoder.rs-0239 */ impl<'a, 'tcx> Metadata<'a, 'tcx> for (CrateMetadataRef<'a>, &'tcx Session) {
/* FP:decoder.rs-0240 */     #[inline]
/* FP:decoder.rs-0241 */     fn blob(self) -> &'a MetadataBlob {
/* FP:decoder.rs-0242 */         &self.0.cdata.blob
/* FP:decoder.rs-0243 */     }
/* FP:decoder.rs-0244 */     #[inline]
/* FP:decoder.rs-0245 */     fn cdata(self) -> Option<CrateMetadataRef<'a>> {
/* FP:decoder.rs-0246 */         Some(self.0)
/* FP:decoder.rs-0247 */     }
/* FP:decoder.rs-0248 */     #[inline]
/* FP:decoder.rs-0249 */     fn sess(self) -> Option<&'tcx Session> {
/* FP:decoder.rs-0250 */         Some(self.1)
/* FP:decoder.rs-0251 */     }
/* FP:decoder.rs-0252 */ }
/* FP:decoder.rs-0253 */ 
/* FP:decoder.rs-0254 */ impl<'a, 'tcx> Metadata<'a, 'tcx> for (CrateMetadataRef<'a>, TyCtxt<'tcx>) {
/* FP:decoder.rs-0255 */     #[inline]
/* FP:decoder.rs-0256 */     fn blob(self) -> &'a MetadataBlob {
/* FP:decoder.rs-0257 */         &self.0.cdata.blob
/* FP:decoder.rs-0258 */     }
/* FP:decoder.rs-0259 */     #[inline]
/* FP:decoder.rs-0260 */     fn cdata(self) -> Option<CrateMetadataRef<'a>> {
/* FP:decoder.rs-0261 */         Some(self.0)
/* FP:decoder.rs-0262 */     }
/* FP:decoder.rs-0263 */     #[inline]
/* FP:decoder.rs-0264 */     fn tcx(self) -> Option<TyCtxt<'tcx>> {
/* FP:decoder.rs-0265 */         Some(self.1)
/* FP:decoder.rs-0266 */     }
/* FP:decoder.rs-0267 */ }
/* FP:decoder.rs-0268 */ 
/* FP:decoder.rs-0269 */ impl<T: ParameterizedOverTcx> LazyValue<T> {
/* FP:decoder.rs-0270 */     #[inline]
/* FP:decoder.rs-0271 */     fn decode<'a, 'tcx, M: Metadata<'a, 'tcx>>(self, metadata: M) -> T::Value<'tcx>
/* FP:decoder.rs-0272 */     where
/* FP:decoder.rs-0273 */         T::Value<'tcx>: Decodable<DecodeContext<'a, 'tcx>>,
/* FP:decoder.rs-0274 */     {
/* FP:decoder.rs-0275 */         let mut dcx = metadata.decoder(self.position.get());
/* FP:decoder.rs-0276 */         dcx.lazy_state = LazyState::NodeStart(self.position);
/* FP:decoder.rs-0277 */         T::Value::decode(&mut dcx)
/* FP:decoder.rs-0278 */     }
/* FP:decoder.rs-0279 */ }
/* FP:decoder.rs-0280 */ 
/* FP:decoder.rs-0281 */ struct DecodeIterator<'a, 'tcx, T> {
/* FP:decoder.rs-0282 */     elem_counter: std::ops::Range<usize>,
/* FP:decoder.rs-0283 */     dcx: DecodeContext<'a, 'tcx>,
/* FP:decoder.rs-0284 */     _phantom: PhantomData<fn() -> T>,
/* FP:decoder.rs-0285 */ }
/* FP:decoder.rs-0286 */ 
/* FP:decoder.rs-0287 */ impl<'a, 'tcx, T: Decodable<DecodeContext<'a, 'tcx>>> Iterator for DecodeIterator<'a, 'tcx, T> {
/* FP:decoder.rs-0288 */     type Item = T;
/* FP:decoder.rs-0289 */ 
/* FP:decoder.rs-0290 */     #[inline(always)]
/* FP:decoder.rs-0291 */     fn next(&mut self) -> Option<Self::Item> {
/* FP:decoder.rs-0292 */         self.elem_counter.next().map(|_| T::decode(&mut self.dcx))
/* FP:decoder.rs-0293 */     }
/* FP:decoder.rs-0294 */ 
/* FP:decoder.rs-0295 */     #[inline(always)]
/* FP:decoder.rs-0296 */     fn size_hint(&self) -> (usize, Option<usize>) {
/* FP:decoder.rs-0297 */         self.elem_counter.size_hint()
/* FP:decoder.rs-0298 */     }
/* FP:decoder.rs-0299 */ }
/* FP:decoder.rs-0300 */ 
/* FP:decoder.rs-0301 */ impl<'a, 'tcx, T: Decodable<DecodeContext<'a, 'tcx>>> ExactSizeIterator
/* FP:decoder.rs-0302 */     for DecodeIterator<'a, 'tcx, T>
/* FP:decoder.rs-0303 */ {
/* FP:decoder.rs-0304 */     fn len(&self) -> usize {
/* FP:decoder.rs-0305 */         self.elem_counter.len()
/* FP:decoder.rs-0306 */     }
/* FP:decoder.rs-0307 */ }
/* FP:decoder.rs-0308 */ 
/* FP:decoder.rs-0309 */ unsafe impl<'a, 'tcx, T: Decodable<DecodeContext<'a, 'tcx>>> TrustedLen
/* FP:decoder.rs-0310 */     for DecodeIterator<'a, 'tcx, T>
/* FP:decoder.rs-0311 */ {
/* FP:decoder.rs-0312 */ }
/* FP:decoder.rs-0313 */ 
/* FP:decoder.rs-0314 */ impl<T: ParameterizedOverTcx> LazyArray<T> {
/* FP:decoder.rs-0315 */     #[inline]
/* FP:decoder.rs-0316 */     fn decode<'a, 'tcx, M: Metadata<'a, 'tcx>>(
/* FP:decoder.rs-0317 */         self,
/* FP:decoder.rs-0318 */         metadata: M,
/* FP:decoder.rs-0319 */     ) -> DecodeIterator<'a, 'tcx, T::Value<'tcx>>
/* FP:decoder.rs-0320 */     where
/* FP:decoder.rs-0321 */         T::Value<'tcx>: Decodable<DecodeContext<'a, 'tcx>>,
/* FP:decoder.rs-0322 */     {
/* FP:decoder.rs-0323 */         let mut dcx = metadata.decoder(self.position.get());
/* FP:decoder.rs-0324 */         dcx.lazy_state = LazyState::NodeStart(self.position);
/* FP:decoder.rs-0325 */         DecodeIterator { elem_counter: (0..self.num_elems), dcx, _phantom: PhantomData }
/* FP:decoder.rs-0326 */     }
/* FP:decoder.rs-0327 */ }
/* FP:decoder.rs-0328 */ 
/* FP:decoder.rs-0329 */ impl<'a, 'tcx> DecodeContext<'a, 'tcx> {
/* FP:decoder.rs-0330 */     #[inline]
/* FP:decoder.rs-0331 */     fn tcx(&self) -> TyCtxt<'tcx> {
/* FP:decoder.rs-0332 */         let Some(tcx) = self.tcx else {
/* FP:decoder.rs-0333 */             bug!(
/* FP:decoder.rs-0334 */                 "No TyCtxt found for decoding. \
/* FP:decoder.rs-0335 */                 You need to explicitly pass `(crate_metadata_ref, tcx)` to `decode` instead of just `crate_metadata_ref`."
/* FP:decoder.rs-0336 */             );
/* FP:decoder.rs-0337 */         };
/* FP:decoder.rs-0338 */         tcx
/* FP:decoder.rs-0339 */     }
/* FP:decoder.rs-0340 */ 
/* FP:decoder.rs-0341 */     #[inline]
/* FP:decoder.rs-0342 */     pub(crate) fn blob(&self) -> &'a MetadataBlob {
/* FP:decoder.rs-0343 */         self.blob
/* FP:decoder.rs-0344 */     }
/* FP:decoder.rs-0345 */ 
/* FP:decoder.rs-0346 */     #[inline]
/* FP:decoder.rs-0347 */     fn cdata(&self) -> CrateMetadataRef<'a> {
/* FP:decoder.rs-0348 */         debug_assert!(self.cdata.is_some(), "missing CrateMetadata in DecodeContext");
/* FP:decoder.rs-0349 */         self.cdata.unwrap()
/* FP:decoder.rs-0350 */     }
/* FP:decoder.rs-0351 */ 
/* FP:decoder.rs-0352 */     #[inline]
/* FP:decoder.rs-0353 */     fn map_encoded_cnum_to_current(&self, cnum: CrateNum) -> CrateNum {
/* FP:decoder.rs-0354 */         self.cdata().map_encoded_cnum_to_current(cnum)
/* FP:decoder.rs-0355 */     }
/* FP:decoder.rs-0356 */ 
/* FP:decoder.rs-0357 */     #[inline]
/* FP:decoder.rs-0358 */     fn read_lazy_offset_then<T>(&mut self, f: impl Fn(NonZero<usize>) -> T) -> T {
/* FP:decoder.rs-0359 */         let distance = self.read_usize();
/* FP:decoder.rs-0360 */         let position = match self.lazy_state {
/* FP:decoder.rs-0361 */             LazyState::NoNode => bug!("read_lazy_with_meta: outside of a metadata node"),
/* FP:decoder.rs-0362 */             LazyState::NodeStart(start) => {
/* FP:decoder.rs-0363 */                 let start = start.get();
/* FP:decoder.rs-0364 */                 assert!(distance <= start);
/* FP:decoder.rs-0365 */                 start - distance
/* FP:decoder.rs-0366 */             }
/* FP:decoder.rs-0367 */             LazyState::Previous(last_pos) => last_pos.get() + distance,
/* FP:decoder.rs-0368 */         };
/* FP:decoder.rs-0369 */         let position = NonZero::new(position).unwrap();
/* FP:decoder.rs-0370 */         self.lazy_state = LazyState::Previous(position);
/* FP:decoder.rs-0371 */         f(position)
/* FP:decoder.rs-0372 */     }
/* FP:decoder.rs-0373 */ 
/* FP:decoder.rs-0374 */     fn read_lazy<T>(&mut self) -> LazyValue<T> {
/* FP:decoder.rs-0375 */         self.read_lazy_offset_then(|pos| LazyValue::from_position(pos))
/* FP:decoder.rs-0376 */     }
/* FP:decoder.rs-0377 */ 
/* FP:decoder.rs-0378 */     fn read_lazy_array<T>(&mut self, len: usize) -> LazyArray<T> {
/* FP:decoder.rs-0379 */         self.read_lazy_offset_then(|pos| LazyArray::from_position_and_num_elems(pos, len))
/* FP:decoder.rs-0380 */     }
/* FP:decoder.rs-0381 */ 
/* FP:decoder.rs-0382 */     fn read_lazy_table<I, T>(&mut self, width: usize, len: usize) -> LazyTable<I, T> {
/* FP:decoder.rs-0383 */         self.read_lazy_offset_then(|pos| LazyTable::from_position_and_encoded_size(pos, width, len))
/* FP:decoder.rs-0384 */     }
/* FP:decoder.rs-0385 */ 
/* FP:decoder.rs-0386 */     #[inline]
/* FP:decoder.rs-0387 */     fn read_raw_bytes(&mut self, len: usize) -> &[u8] {
/* FP:decoder.rs-0388 */         self.opaque.read_raw_bytes(len)
/* FP:decoder.rs-0389 */     }
/* FP:decoder.rs-0390 */ 
/* FP:decoder.rs-0391 */     fn decode_symbol_or_byte_symbol<S>(
/* FP:decoder.rs-0392 */         &mut self,
/* FP:decoder.rs-0393 */         new_from_index: impl Fn(u32) -> S,
/* FP:decoder.rs-0394 */         read_and_intern_str_or_byte_str_this: impl Fn(&mut Self) -> S,
/* FP:decoder.rs-0395 */         read_and_intern_str_or_byte_str_opaque: impl Fn(&mut MemDecoder<'a>) -> S,
/* FP:decoder.rs-0396 */     ) -> S {
/* FP:decoder.rs-0397 */         let tag = self.read_u8();
/* FP:decoder.rs-0398 */ 
/* FP:decoder.rs-0399 */         match tag {
/* FP:decoder.rs-0400 */             SYMBOL_STR => read_and_intern_str_or_byte_str_this(self),
/* FP:decoder.rs-0401 */             SYMBOL_OFFSET => {
/* FP:decoder.rs-0402 */                 // read str offset
/* FP:decoder.rs-0403 */                 let pos = self.read_usize();
/* FP:decoder.rs-0404 */ 
/* FP:decoder.rs-0405 */                 // move to str offset and read
/* FP:decoder.rs-0406 */                 self.opaque.with_position(pos, |d| read_and_intern_str_or_byte_str_opaque(d))
/* FP:decoder.rs-0407 */             }
/* FP:decoder.rs-0408 */             SYMBOL_PREDEFINED => new_from_index(self.read_u32()),
/* FP:decoder.rs-0409 */             _ => unreachable!(),
/* FP:decoder.rs-0410 */         }
/* FP:decoder.rs-0411 */     }
/* FP:decoder.rs-0412 */ }
/* FP:decoder.rs-0413 */ 
/* FP:decoder.rs-0414 */ impl<'a, 'tcx> TyDecoder<'tcx> for DecodeContext<'a, 'tcx> {
/* FP:decoder.rs-0415 */     const CLEAR_CROSS_CRATE: bool = true;
/* FP:decoder.rs-0416 */ 
/* FP:decoder.rs-0417 */     #[inline]
/* FP:decoder.rs-0418 */     fn interner(&self) -> TyCtxt<'tcx> {
/* FP:decoder.rs-0419 */         self.tcx()
/* FP:decoder.rs-0420 */     }
/* FP:decoder.rs-0421 */ 
/* FP:decoder.rs-0422 */     fn cached_ty_for_shorthand<F>(&mut self, shorthand: usize, or_insert_with: F) -> Ty<'tcx>
/* FP:decoder.rs-0423 */     where
/* FP:decoder.rs-0424 */         F: FnOnce(&mut Self) -> Ty<'tcx>,
/* FP:decoder.rs-0425 */     {
/* FP:decoder.rs-0426 */         let tcx = self.tcx();
/* FP:decoder.rs-0427 */ 
/* FP:decoder.rs-0428 */         let key = ty::CReaderCacheKey { cnum: Some(self.cdata().cnum), pos: shorthand };
/* FP:decoder.rs-0429 */ 
/* FP:decoder.rs-0430 */         if let Some(&ty) = tcx.ty_rcache.borrow().get(&key) {
/* FP:decoder.rs-0431 */             return ty;
/* FP:decoder.rs-0432 */         }
/* FP:decoder.rs-0433 */ 
/* FP:decoder.rs-0434 */         let ty = or_insert_with(self);
/* FP:decoder.rs-0435 */         tcx.ty_rcache.borrow_mut().insert(key, ty);
/* FP:decoder.rs-0436 */         ty
/* FP:decoder.rs-0437 */     }
/* FP:decoder.rs-0438 */ 
/* FP:decoder.rs-0439 */     fn with_position<F, R>(&mut self, pos: usize, f: F) -> R
/* FP:decoder.rs-0440 */     where
/* FP:decoder.rs-0441 */         F: FnOnce(&mut Self) -> R,
/* FP:decoder.rs-0442 */     {
/* FP:decoder.rs-0443 */         let new_opaque = self.opaque.split_at(pos);
/* FP:decoder.rs-0444 */         let old_opaque = mem::replace(&mut self.opaque, new_opaque);
/* FP:decoder.rs-0445 */         let old_state = mem::replace(&mut self.lazy_state, LazyState::NoNode);
/* FP:decoder.rs-0446 */         let r = f(self);
/* FP:decoder.rs-0447 */         self.opaque = old_opaque;
/* FP:decoder.rs-0448 */         self.lazy_state = old_state;
/* FP:decoder.rs-0449 */         r
/* FP:decoder.rs-0450 */     }
/* FP:decoder.rs-0451 */ 
/* FP:decoder.rs-0452 */     fn decode_alloc_id(&mut self) -> crate::rustc_middle::mir::interpret::AllocId {
/* FP:decoder.rs-0453 */         if let Some(alloc_decoding_session) = self.alloc_decoding_session {
/* FP:decoder.rs-0454 */             alloc_decoding_session.decode_alloc_id(self)
/* FP:decoder.rs-0455 */         } else {
/* FP:decoder.rs-0456 */             bug!("Attempting to decode interpret::AllocId without CrateMetadata")
/* FP:decoder.rs-0457 */         }
/* FP:decoder.rs-0458 */     }
/* FP:decoder.rs-0459 */ }
/* FP:decoder.rs-0460 */ 
/* FP:decoder.rs-0461 */ impl<'a, 'tcx> Decodable<DecodeContext<'a, 'tcx>> for ExpnIndex {
/* FP:decoder.rs-0462 */     #[inline]
/* FP:decoder.rs-0463 */     fn decode(d: &mut DecodeContext<'a, 'tcx>) -> ExpnIndex {
/* FP:decoder.rs-0464 */         ExpnIndex::from_u32(d.read_u32())
/* FP:decoder.rs-0465 */     }
/* FP:decoder.rs-0466 */ }
/* FP:decoder.rs-0467 */ 
/* FP:decoder.rs-0468 */ impl<'a, 'tcx> SpanDecoder for DecodeContext<'a, 'tcx> {
/* FP:decoder.rs-0469 */     fn decode_attr_id(&mut self) -> crate::rustc_span::AttrId {
/* FP:decoder.rs-0470 */         let sess = self.sess.expect("can't decode AttrId without Session");
/* FP:decoder.rs-0471 */         sess.psess.attr_id_generator.mk_attr_id()
/* FP:decoder.rs-0472 */     }
/* FP:decoder.rs-0473 */ 
/* FP:decoder.rs-0474 */     fn decode_crate_num(&mut self) -> CrateNum {
/* FP:decoder.rs-0475 */         let cnum = CrateNum::from_u32(self.read_u32());
/* FP:decoder.rs-0476 */         self.map_encoded_cnum_to_current(cnum)
/* FP:decoder.rs-0477 */     }
/* FP:decoder.rs-0478 */ 
/* FP:decoder.rs-0479 */     fn decode_def_index(&mut self) -> DefIndex {
/* FP:decoder.rs-0480 */         DefIndex::from_u32(self.read_u32())
/* FP:decoder.rs-0481 */     }
/* FP:decoder.rs-0482 */ 
/* FP:decoder.rs-0483 */     fn decode_def_id(&mut self) -> DefId {
/* FP:decoder.rs-0484 */         DefId { krate: Decodable::decode(self), index: Decodable::decode(self) }
/* FP:decoder.rs-0485 */     }
/* FP:decoder.rs-0486 */ 
/* FP:decoder.rs-0487 */     fn decode_syntax_context(&mut self) -> SyntaxContext {
/* FP:decoder.rs-0488 */         let cdata = self.cdata();
/* FP:decoder.rs-0489 */ 
/* FP:decoder.rs-0490 */         let Some(sess) = self.sess else {
/* FP:decoder.rs-0491 */             bug!(
/* FP:decoder.rs-0492 */                 "Cannot decode SyntaxContext without Session.\
/* FP:decoder.rs-0493 */                 You need to explicitly pass `(crate_metadata_ref, tcx)` to `decode` instead of just `crate_metadata_ref`."
/* FP:decoder.rs-0494 */             );
/* FP:decoder.rs-0495 */         };
/* FP:decoder.rs-0496 */ 
/* FP:decoder.rs-0497 */         let cname = cdata.root.name();
/* FP:decoder.rs-0498 */         crate::rustc_span::hygiene::decode_syntax_context(self, &cdata.hygiene_context, |_, id| {
/* FP:decoder.rs-0499 */             debug!("SpecializedDecoder<SyntaxContext>: decoding {}", id);
/* FP:decoder.rs-0500 */             cdata
/* FP:decoder.rs-0501 */                 .root
/* FP:decoder.rs-0502 */                 .syntax_contexts
/* FP:decoder.rs-0503 */                 .get(cdata, id)
/* FP:decoder.rs-0504 */                 .unwrap_or_else(|| panic!("Missing SyntaxContext {id:?} for crate {cname:?}"))
/* FP:decoder.rs-0505 */                 .decode((cdata, sess))
/* FP:decoder.rs-0506 */         })
/* FP:decoder.rs-0507 */     }
/* FP:decoder.rs-0508 */ 
/* FP:decoder.rs-0509 */     fn decode_expn_id(&mut self) -> ExpnId {
/* FP:decoder.rs-0510 */         let local_cdata = self.cdata();
/* FP:decoder.rs-0511 */ 
/* FP:decoder.rs-0512 */         let Some(sess) = self.sess else {
/* FP:decoder.rs-0513 */             bug!(
/* FP:decoder.rs-0514 */                 "Cannot decode ExpnId without Session. \
/* FP:decoder.rs-0515 */                 You need to explicitly pass `(crate_metadata_ref, tcx)` to `decode` instead of just `crate_metadata_ref`."
/* FP:decoder.rs-0516 */             );
/* FP:decoder.rs-0517 */         };
/* FP:decoder.rs-0518 */ 
/* FP:decoder.rs-0519 */         let cnum = CrateNum::decode(self);
/* FP:decoder.rs-0520 */         let index = u32::decode(self);
/* FP:decoder.rs-0521 */ 
/* FP:decoder.rs-0522 */         let expn_id = crate::rustc_span::hygiene::decode_expn_id(cnum, index, |expn_id| {
/* FP:decoder.rs-0523 */             let ExpnId { krate: cnum, local_id: index } = expn_id;
/* FP:decoder.rs-0524 */             // Lookup local `ExpnData`s in our own crate data. Foreign `ExpnData`s
/* FP:decoder.rs-0525 */             // are stored in the owning crate, to avoid duplication.
/* FP:decoder.rs-0526 */             debug_assert_ne!(cnum, LOCAL_CRATE);
/* FP:decoder.rs-0527 */             let crate_data = if cnum == local_cdata.cnum {
/* FP:decoder.rs-0528 */                 local_cdata
/* FP:decoder.rs-0529 */             } else {
/* FP:decoder.rs-0530 */                 local_cdata.cstore.get_crate_data(cnum)
/* FP:decoder.rs-0531 */             };
/* FP:decoder.rs-0532 */             let expn_data = crate_data
/* FP:decoder.rs-0533 */                 .root
/* FP:decoder.rs-0534 */                 .expn_data
/* FP:decoder.rs-0535 */                 .get(crate_data, index)
/* FP:decoder.rs-0536 */                 .unwrap()
/* FP:decoder.rs-0537 */                 .decode((crate_data, sess));
/* FP:decoder.rs-0538 */             let expn_hash = crate_data
/* FP:decoder.rs-0539 */                 .root
/* FP:decoder.rs-0540 */                 .expn_hashes
/* FP:decoder.rs-0541 */                 .get(crate_data, index)
/* FP:decoder.rs-0542 */                 .unwrap()
/* FP:decoder.rs-0543 */                 .decode((crate_data, sess));
/* FP:decoder.rs-0544 */             (expn_data, expn_hash)
/* FP:decoder.rs-0545 */         });
/* FP:decoder.rs-0546 */         expn_id
/* FP:decoder.rs-0547 */     }
/* FP:decoder.rs-0548 */ 
/* FP:decoder.rs-0549 */     fn decode_span(&mut self) -> Span {
/* FP:decoder.rs-0550 */         let start = self.position();
/* FP:decoder.rs-0551 */         let tag = SpanTag(self.peek_byte());
/* FP:decoder.rs-0552 */         let data = if tag.kind() == SpanKind::Indirect {
/* FP:decoder.rs-0553 */             // Skip past the tag we just peek'd.
/* FP:decoder.rs-0554 */             self.read_u8();
/* FP:decoder.rs-0555 */             // indirect tag lengths are safe to access, since they're (0, 8)
/* FP:decoder.rs-0556 */             let bytes_needed = tag.length().unwrap().0 as usize;
/* FP:decoder.rs-0557 */             let mut total = [0u8; usize::BITS as usize / 8];
/* FP:decoder.rs-0558 */             total[..bytes_needed].copy_from_slice(self.read_raw_bytes(bytes_needed));
/* FP:decoder.rs-0559 */             let offset_or_position = usize::from_le_bytes(total);
/* FP:decoder.rs-0560 */             let position = if tag.is_relative_offset() {
/* FP:decoder.rs-0561 */                 start - offset_or_position
/* FP:decoder.rs-0562 */             } else {
/* FP:decoder.rs-0563 */                 offset_or_position
/* FP:decoder.rs-0564 */             };
/* FP:decoder.rs-0565 */             self.with_position(position, SpanData::decode)
/* FP:decoder.rs-0566 */         } else {
/* FP:decoder.rs-0567 */             SpanData::decode(self)
/* FP:decoder.rs-0568 */         };
/* FP:decoder.rs-0569 */         data.span()
/* FP:decoder.rs-0570 */     }
/* FP:decoder.rs-0571 */ 
/* FP:decoder.rs-0572 */     fn decode_symbol(&mut self) -> Symbol {
/* FP:decoder.rs-0573 */         self.decode_symbol_or_byte_symbol(
/* FP:decoder.rs-0574 */             Symbol::new,
/* FP:decoder.rs-0575 */             |this| Symbol::intern(this.read_str()),
/* FP:decoder.rs-0576 */             |opaque| Symbol::intern(opaque.read_str()),
/* FP:decoder.rs-0577 */         )
/* FP:decoder.rs-0578 */     }
/* FP:decoder.rs-0579 */ 
/* FP:decoder.rs-0580 */     fn decode_byte_symbol(&mut self) -> ByteSymbol {
/* FP:decoder.rs-0581 */         self.decode_symbol_or_byte_symbol(
/* FP:decoder.rs-0582 */             ByteSymbol::new,
/* FP:decoder.rs-0583 */             |this| ByteSymbol::intern(this.read_byte_str()),
/* FP:decoder.rs-0584 */             |opaque| ByteSymbol::intern(opaque.read_byte_str()),
/* FP:decoder.rs-0585 */         )
/* FP:decoder.rs-0586 */     }
/* FP:decoder.rs-0587 */ }
/* FP:decoder.rs-0588 */ 
/* FP:decoder.rs-0589 */ impl<'a, 'tcx> Decodable<DecodeContext<'a, 'tcx>> for SpanData {
/* FP:decoder.rs-0590 */     fn decode(decoder: &mut DecodeContext<'a, 'tcx>) -> SpanData {
/* FP:decoder.rs-0591 */         let tag = SpanTag::decode(decoder);
/* FP:decoder.rs-0592 */         let ctxt = tag.context().unwrap_or_else(|| SyntaxContext::decode(decoder));
/* FP:decoder.rs-0593 */ 
/* FP:decoder.rs-0594 */         if tag.kind() == SpanKind::Partial {
/* FP:decoder.rs-0595 */             return DUMMY_SP.with_ctxt(ctxt).data();
/* FP:decoder.rs-0596 */         }
/* FP:decoder.rs-0597 */ 
/* FP:decoder.rs-0598 */         debug_assert!(tag.kind() == SpanKind::Local || tag.kind() == SpanKind::Foreign);
/* FP:decoder.rs-0599 */ 
/* FP:decoder.rs-0600 */         let lo = BytePos::decode(decoder);
/* FP:decoder.rs-0601 */         let len = tag.length().unwrap_or_else(|| BytePos::decode(decoder));
/* FP:decoder.rs-0602 */         let hi = lo + len;
/* FP:decoder.rs-0603 */ 
/* FP:decoder.rs-0604 */         let Some(sess) = decoder.sess else {
/* FP:decoder.rs-0605 */             bug!(
/* FP:decoder.rs-0606 */                 "Cannot decode Span without Session. \
/* FP:decoder.rs-0607 */                 You need to explicitly pass `(crate_metadata_ref, tcx)` to `decode` instead of just `crate_metadata_ref`."
/* FP:decoder.rs-0608 */             )
/* FP:decoder.rs-0609 */         };
/* FP:decoder.rs-0610 */ 
/* FP:decoder.rs-0611 */         // Index of the file in the corresponding crate's list of encoded files.
/* FP:decoder.rs-0612 */         let metadata_index = u32::decode(decoder);
/* FP:decoder.rs-0613 */ 
/* FP:decoder.rs-0614 */         // There are two possibilities here:
/* FP:decoder.rs-0615 */         // 1. This is a 'local span', which is located inside a `SourceFile`
/* FP:decoder.rs-0616 */         // that came from this crate. In this case, we use the source map data
/* FP:decoder.rs-0617 */         // encoded in this crate. This branch should be taken nearly all of the time.
/* FP:decoder.rs-0618 */         // 2. This is a 'foreign span', which is located inside a `SourceFile`
/* FP:decoder.rs-0619 */         // that came from a *different* crate (some crate upstream of the one
/* FP:decoder.rs-0620 */         // whose metadata we're looking at). For example, consider this dependency graph:
/* FP:decoder.rs-0621 */         //
/* FP:decoder.rs-0622 */         // A -> B -> C
/* FP:decoder.rs-0623 */         //
/* FP:decoder.rs-0624 */         // Suppose that we're currently compiling crate A, and start deserializing
/* FP:decoder.rs-0625 */         // metadata from crate B. When we deserialize a Span from crate B's metadata,
/* FP:decoder.rs-0626 */         // there are two possibilities:
/* FP:decoder.rs-0627 */         //
/* FP:decoder.rs-0628 */         // 1. The span references a file from crate B. This makes it a 'local' span,
/* FP:decoder.rs-0629 */         // which means that we can use crate B's serialized source map information.
/* FP:decoder.rs-0630 */         // 2. The span references a file from crate C. This makes it a 'foreign' span,
/* FP:decoder.rs-0631 */         // which means we need to use Crate *C* (not crate B) to determine the source
/* FP:decoder.rs-0632 */         // map information. We only record source map information for a file in the
/* FP:decoder.rs-0633 */         // crate that 'owns' it, so deserializing a Span may require us to look at
/* FP:decoder.rs-0634 */         // a transitive dependency.
/* FP:decoder.rs-0635 */         //
/* FP:decoder.rs-0636 */         // When we encode a foreign span, we adjust its 'lo' and 'high' values
/* FP:decoder.rs-0637 */         // to be based on the *foreign* crate (e.g. crate C), not the crate
/* FP:decoder.rs-0638 */         // we are writing metadata for (e.g. crate B). This allows us to
/* FP:decoder.rs-0639 */         // treat the 'local' and 'foreign' cases almost identically during deserialization:
/* FP:decoder.rs-0640 */         // we can call `imported_source_file` for the proper crate, and binary search
/* FP:decoder.rs-0641 */         // through the returned slice using our span.
/* FP:decoder.rs-0642 */         let source_file = if tag.kind() == SpanKind::Local {
/* FP:decoder.rs-0643 */             decoder.cdata().imported_source_file(metadata_index, sess)
/* FP:decoder.rs-0644 */         } else {
/* FP:decoder.rs-0645 */             // When we encode a proc-macro crate, all `Span`s should be encoded
/* FP:decoder.rs-0646 */             // with `TAG_VALID_SPAN_LOCAL`
/* FP:decoder.rs-0647 */             if decoder.cdata().root.is_proc_macro_crate() {
/* FP:decoder.rs-0648 */                 // Decode `CrateNum` as u32 - using `CrateNum::decode` will ICE
/* FP:decoder.rs-0649 */                 // since we don't have `cnum_map` populated.
/* FP:decoder.rs-0650 */                 let cnum = u32::decode(decoder);
/* FP:decoder.rs-0651 */                 panic!(
/* FP:decoder.rs-0652 */                     "Decoding of crate {:?} tried to access proc-macro dep {:?}",
/* FP:decoder.rs-0653 */                     decoder.cdata().root.header.name,
/* FP:decoder.rs-0654 */                     cnum
/* FP:decoder.rs-0655 */                 );
/* FP:decoder.rs-0656 */             }
/* FP:decoder.rs-0657 */             // tag is TAG_VALID_SPAN_FOREIGN, checked by `debug_assert` above
/* FP:decoder.rs-0658 */             let cnum = CrateNum::decode(decoder);
/* FP:decoder.rs-0659 */             debug!(
/* FP:decoder.rs-0660 */                 "SpecializedDecoder<Span>::specialized_decode: loading source files from cnum {:?}",
/* FP:decoder.rs-0661 */                 cnum
/* FP:decoder.rs-0662 */             );
/* FP:decoder.rs-0663 */ 
/* FP:decoder.rs-0664 */             let foreign_data = decoder.cdata().cstore.get_crate_data(cnum);
/* FP:decoder.rs-0665 */             foreign_data.imported_source_file(metadata_index, sess)
/* FP:decoder.rs-0666 */         };
/* FP:decoder.rs-0667 */ 
/* FP:decoder.rs-0668 */         // Make sure our span is well-formed.
/* FP:decoder.rs-0669 */         debug_assert!(
/* FP:decoder.rs-0670 */             lo + source_file.original_start_pos <= source_file.original_end_pos,
/* FP:decoder.rs-0671 */             "Malformed encoded span: lo={:?} source_file.original_start_pos={:?} source_file.original_end_pos={:?}",
/* FP:decoder.rs-0672 */             lo,
/* FP:decoder.rs-0673 */             source_file.original_start_pos,
/* FP:decoder.rs-0674 */             source_file.original_end_pos
/* FP:decoder.rs-0675 */         );
/* FP:decoder.rs-0676 */ 
/* FP:decoder.rs-0677 */         // Make sure we correctly filtered out invalid spans during encoding.
/* FP:decoder.rs-0678 */         debug_assert!(
/* FP:decoder.rs-0679 */             hi + source_file.original_start_pos <= source_file.original_end_pos,
/* FP:decoder.rs-0680 */             "Malformed encoded span: hi={:?} source_file.original_start_pos={:?} source_file.original_end_pos={:?}",
/* FP:decoder.rs-0681 */             hi,
/* FP:decoder.rs-0682 */             source_file.original_start_pos,
/* FP:decoder.rs-0683 */             source_file.original_end_pos
/* FP:decoder.rs-0684 */         );
/* FP:decoder.rs-0685 */ 
/* FP:decoder.rs-0686 */         let lo = lo + source_file.translated_source_file.start_pos;
/* FP:decoder.rs-0687 */         let hi = hi + source_file.translated_source_file.start_pos;
/* FP:decoder.rs-0688 */ 
/* FP:decoder.rs-0689 */         // Do not try to decode parent for foreign spans (it wasn't encoded in the first place).
/* FP:decoder.rs-0690 */         SpanData { lo, hi, ctxt, parent: None }
/* FP:decoder.rs-0691 */     }
/* FP:decoder.rs-0692 */ }
/* FP:decoder.rs-0693 */ 
/* FP:decoder.rs-0694 */ impl<'a, 'tcx> Decodable<DecodeContext<'a, 'tcx>> for &'tcx [(ty::Clause<'tcx>, Span)] {
/* FP:decoder.rs-0695 */     fn decode(d: &mut DecodeContext<'a, 'tcx>) -> Self {
/* FP:decoder.rs-0696 */         ty::codec::RefDecodable::decode(d)
/* FP:decoder.rs-0697 */     }
/* FP:decoder.rs-0698 */ }
/* FP:decoder.rs-0699 */ 
/* FP:decoder.rs-0700 */ impl<'a, 'tcx, T> Decodable<DecodeContext<'a, 'tcx>> for LazyValue<T> {
/* FP:decoder.rs-0701 */     fn decode(decoder: &mut DecodeContext<'a, 'tcx>) -> Self {
/* FP:decoder.rs-0702 */         decoder.read_lazy()
/* FP:decoder.rs-0703 */     }
/* FP:decoder.rs-0704 */ }
/* FP:decoder.rs-0705 */ 
/* FP:decoder.rs-0706 */ impl<'a, 'tcx, T> Decodable<DecodeContext<'a, 'tcx>> for LazyArray<T> {
/* FP:decoder.rs-0707 */     #[inline]
/* FP:decoder.rs-0708 */     fn decode(decoder: &mut DecodeContext<'a, 'tcx>) -> Self {
/* FP:decoder.rs-0709 */         let len = decoder.read_usize();
/* FP:decoder.rs-0710 */         if len == 0 { LazyArray::default() } else { decoder.read_lazy_array(len) }
/* FP:decoder.rs-0711 */     }
/* FP:decoder.rs-0712 */ }
/* FP:decoder.rs-0713 */ 
/* FP:decoder.rs-0714 */ impl<'a, 'tcx, I: Idx, T> Decodable<DecodeContext<'a, 'tcx>> for LazyTable<I, T> {
/* FP:decoder.rs-0715 */     fn decode(decoder: &mut DecodeContext<'a, 'tcx>) -> Self {
/* FP:decoder.rs-0716 */         let width = decoder.read_usize();
/* FP:decoder.rs-0717 */         let len = decoder.read_usize();
/* FP:decoder.rs-0718 */         decoder.read_lazy_table(width, len)
/* FP:decoder.rs-0719 */     }
/* FP:decoder.rs-0720 */ }
/* FP:decoder.rs-0721 */ 
/* FP:decoder.rs-0722 */ implement_ty_decoder!(DecodeContext<'a, 'tcx>);
/* FP:decoder.rs-0723 */ 
/* FP:decoder.rs-0724 */ impl MetadataBlob {
/* FP:decoder.rs-0725 */     pub(crate) fn check_compatibility(
/* FP:decoder.rs-0726 */         &self,
/* FP:decoder.rs-0727 */         cfg_version: &'static str,
/* FP:decoder.rs-0728 */     ) -> Result<(), Option<String>> {
/* FP:decoder.rs-0729 */         if !self.blob().starts_with(METADATA_HEADER) {
/* FP:decoder.rs-0730 */             if self.blob().starts_with(b"rust") {
/* FP:decoder.rs-0731 */                 return Err(Some("<unknown rustc version>".to_owned()));
/* FP:decoder.rs-0732 */             }
/* FP:decoder.rs-0733 */             return Err(None);
/* FP:decoder.rs-0734 */         }
/* FP:decoder.rs-0735 */ 
/* FP:decoder.rs-0736 */         let found_version =
/* FP:decoder.rs-0737 */             LazyValue::<String>::from_position(NonZero::new(METADATA_HEADER.len() + 8).unwrap())
/* FP:decoder.rs-0738 */                 .decode(self);
/* FP:decoder.rs-0739 */         if rustc_version(cfg_version) != found_version {
/* FP:decoder.rs-0740 */             return Err(Some(found_version));
/* FP:decoder.rs-0741 */         }
/* FP:decoder.rs-0742 */ 
/* FP:decoder.rs-0743 */         Ok(())
/* FP:decoder.rs-0744 */     }
/* FP:decoder.rs-0745 */ 
/* FP:decoder.rs-0746 */     fn root_pos(&self) -> NonZero<usize> {
/* FP:decoder.rs-0747 */         let offset = METADATA_HEADER.len();
/* FP:decoder.rs-0748 */         let pos_bytes = self.blob()[offset..][..8].try_into().unwrap();
/* FP:decoder.rs-0749 */         let pos = u64::from_le_bytes(pos_bytes);
/* FP:decoder.rs-0750 */         NonZero::new(pos as usize).unwrap()
/* FP:decoder.rs-0751 */     }
/* FP:decoder.rs-0752 */ 
/* FP:decoder.rs-0753 */     pub(crate) fn get_header(&self) -> CrateHeader {
/* FP:decoder.rs-0754 */         let pos = self.root_pos();
/* FP:decoder.rs-0755 */         LazyValue::<CrateHeader>::from_position(pos).decode(self)
/* FP:decoder.rs-0756 */     }
/* FP:decoder.rs-0757 */ 
/* FP:decoder.rs-0758 */     pub(crate) fn get_root(&self) -> CrateRoot {
/* FP:decoder.rs-0759 */         let pos = self.root_pos();
/* FP:decoder.rs-0760 */         LazyValue::<CrateRoot>::from_position(pos).decode(self)
/* FP:decoder.rs-0761 */     }
/* FP:decoder.rs-0762 */ 
/* FP:decoder.rs-0763 */     pub(crate) fn list_crate_metadata(
/* FP:decoder.rs-0764 */         &self,
/* FP:decoder.rs-0765 */         out: &mut dyn io::Write,
/* FP:decoder.rs-0766 */         ls_kinds: &[String],
/* FP:decoder.rs-0767 */     ) -> io::Result<()> {
/* FP:decoder.rs-0768 */         let root = self.get_root();
/* FP:decoder.rs-0769 */ 
/* FP:decoder.rs-0770 */         let all_ls_kinds = vec![
/* FP:decoder.rs-0771 */             "root".to_owned(),
/* FP:decoder.rs-0772 */             "lang_items".to_owned(),
/* FP:decoder.rs-0773 */             "features".to_owned(),
/* FP:decoder.rs-0774 */             "items".to_owned(),
/* FP:decoder.rs-0775 */         ];
/* FP:decoder.rs-0776 */         let ls_kinds = if ls_kinds.contains(&"all".to_owned()) { &all_ls_kinds } else { ls_kinds };
/* FP:decoder.rs-0777 */ 
/* FP:decoder.rs-0778 */         for kind in ls_kinds {
/* FP:decoder.rs-0779 */             match &**kind {
/* FP:decoder.rs-0780 */                 "root" => {
/* FP:decoder.rs-0781 */                     writeln!(out, "Crate info:")?;
/* FP:decoder.rs-0782 */                     writeln!(out, "name {}{}", root.name(), root.extra_filename)?;
/* FP:decoder.rs-0783 */                     writeln!(
/* FP:decoder.rs-0784 */                         out,
/* FP:decoder.rs-0785 */                         "hash {} stable_crate_id {:?}",
/* FP:decoder.rs-0786 */                         root.hash(),
/* FP:decoder.rs-0787 */                         root.stable_crate_id
/* FP:decoder.rs-0788 */                     )?;
/* FP:decoder.rs-0789 */                     writeln!(out, "proc_macro {:?}", root.proc_macro_data.is_some())?;
/* FP:decoder.rs-0790 */                     writeln!(out, "triple {}", root.header.triple.tuple())?;
/* FP:decoder.rs-0791 */                     writeln!(out, "edition {}", root.edition)?;
/* FP:decoder.rs-0792 */                     writeln!(out, "symbol_mangling_version {:?}", root.symbol_mangling_version)?;
/* FP:decoder.rs-0793 */                     writeln!(
/* FP:decoder.rs-0794 */                         out,
/* FP:decoder.rs-0795 */                         "required_panic_strategy {:?} panic_in_drop_strategy {:?}",
/* FP:decoder.rs-0796 */                         root.required_panic_strategy, root.panic_in_drop_strategy
/* FP:decoder.rs-0797 */                     )?;
/* FP:decoder.rs-0798 */                     writeln!(
/* FP:decoder.rs-0799 */                         out,
/* FP:decoder.rs-0800 */                         "has_global_allocator {} has_alloc_error_handler {} has_panic_handler {} has_default_lib_allocator {}",
/* FP:decoder.rs-0801 */                         root.has_global_allocator,
/* FP:decoder.rs-0802 */                         root.has_alloc_error_handler,
/* FP:decoder.rs-0803 */                         root.has_panic_handler,
/* FP:decoder.rs-0804 */                         root.has_default_lib_allocator
/* FP:decoder.rs-0805 */                     )?;
/* FP:decoder.rs-0806 */                     writeln!(
/* FP:decoder.rs-0807 */                         out,
/* FP:decoder.rs-0808 */                         "compiler_builtins {} needs_allocator {} needs_panic_runtime {} no_builtins {} panic_runtime {} profiler_runtime {}",
/* FP:decoder.rs-0809 */                         root.compiler_builtins,
/* FP:decoder.rs-0810 */                         root.needs_allocator,
/* FP:decoder.rs-0811 */                         root.needs_panic_runtime,
/* FP:decoder.rs-0812 */                         root.no_builtins,
/* FP:decoder.rs-0813 */                         root.panic_runtime,
/* FP:decoder.rs-0814 */                         root.profiler_runtime
/* FP:decoder.rs-0815 */                     )?;
/* FP:decoder.rs-0816 */ 
/* FP:decoder.rs-0817 */                     writeln!(out, "=External Dependencies=")?;
/* FP:decoder.rs-0818 */                     let dylib_dependency_formats =
/* FP:decoder.rs-0819 */                         root.dylib_dependency_formats.decode(self).collect::<Vec<_>>();
/* FP:decoder.rs-0820 */                     for (i, dep) in root.crate_deps.decode(self).enumerate() {
/* FP:decoder.rs-0821 */                         let CrateDep { name, extra_filename, hash, host_hash, kind, is_private } =
/* FP:decoder.rs-0822 */                             dep;
/* FP:decoder.rs-0823 */                         let number = i + 1;
/* FP:decoder.rs-0824 */ 
/* FP:decoder.rs-0825 */                         writeln!(
/* FP:decoder.rs-0826 */                             out,
/* FP:decoder.rs-0827 */                             "{number} {name}{extra_filename} hash {hash} host_hash {host_hash:?} kind {kind:?} {privacy}{linkage}",
/* FP:decoder.rs-0828 */                             privacy = if is_private { "private" } else { "public" },
/* FP:decoder.rs-0829 */                             linkage = if dylib_dependency_formats.is_empty() {
/* FP:decoder.rs-0830 */                                 String::new()
/* FP:decoder.rs-0831 */                             } else {
/* FP:decoder.rs-0832 */                                 format!(" linkage {:?}", dylib_dependency_formats[i])
/* FP:decoder.rs-0833 */                             }
/* FP:decoder.rs-0834 */                         )?;
/* FP:decoder.rs-0835 */                     }
/* FP:decoder.rs-0836 */                     write!(out, "\n")?;
/* FP:decoder.rs-0837 */                 }
/* FP:decoder.rs-0838 */ 
/* FP:decoder.rs-0839 */                 "lang_items" => {
/* FP:decoder.rs-0840 */                     writeln!(out, "=Lang items=")?;
/* FP:decoder.rs-0841 */                     for (id, lang_item) in root.lang_items.decode(self) {
/* FP:decoder.rs-0842 */                         writeln!(
/* FP:decoder.rs-0843 */                             out,
/* FP:decoder.rs-0844 */                             "{} = crate{}",
/* FP:decoder.rs-0845 */                             lang_item.name(),
/* FP:decoder.rs-0846 */                             DefPath::make(LOCAL_CRATE, id, |parent| root
/* FP:decoder.rs-0847 */                                 .tables
/* FP:decoder.rs-0848 */                                 .def_keys
/* FP:decoder.rs-0849 */                                 .get(self, parent)
/* FP:decoder.rs-0850 */                                 .unwrap()
/* FP:decoder.rs-0851 */                                 .decode(self))
/* FP:decoder.rs-0852 */                             .to_string_no_crate_verbose()
/* FP:decoder.rs-0853 */                         )?;
/* FP:decoder.rs-0854 */                     }
/* FP:decoder.rs-0855 */                     for lang_item in root.lang_items_missing.decode(self) {
/* FP:decoder.rs-0856 */                         writeln!(out, "{} = <missing>", lang_item.name())?;
/* FP:decoder.rs-0857 */                     }
/* FP:decoder.rs-0858 */                     write!(out, "\n")?;
/* FP:decoder.rs-0859 */                 }
/* FP:decoder.rs-0860 */ 
/* FP:decoder.rs-0861 */                 "features" => {
/* FP:decoder.rs-0862 */                     writeln!(out, "=Lib features=")?;
/* FP:decoder.rs-0863 */                     for (feature, since) in root.lib_features.decode(self) {
/* FP:decoder.rs-0864 */                         writeln!(
/* FP:decoder.rs-0865 */                             out,
/* FP:decoder.rs-0866 */                             "{}{}",
/* FP:decoder.rs-0867 */                             feature,
/* FP:decoder.rs-0868 */                             if let FeatureStability::AcceptedSince(since) = since {
/* FP:decoder.rs-0869 */                                 format!(" since {since}")
/* FP:decoder.rs-0870 */                             } else {
/* FP:decoder.rs-0871 */                                 String::new()
/* FP:decoder.rs-0872 */                             }
/* FP:decoder.rs-0873 */                         )?;
/* FP:decoder.rs-0874 */                     }
/* FP:decoder.rs-0875 */                     write!(out, "\n")?;
/* FP:decoder.rs-0876 */                 }
/* FP:decoder.rs-0877 */ 
/* FP:decoder.rs-0878 */                 "items" => {
/* FP:decoder.rs-0879 */                     writeln!(out, "=Items=")?;
/* FP:decoder.rs-0880 */ 
/* FP:decoder.rs-0881 */                     fn print_item(
/* FP:decoder.rs-0882 */                         blob: &MetadataBlob,
/* FP:decoder.rs-0883 */                         out: &mut dyn io::Write,
/* FP:decoder.rs-0884 */                         item: DefIndex,
/* FP:decoder.rs-0885 */                         indent: usize,
/* FP:decoder.rs-0886 */                     ) -> io::Result<()> {
/* FP:decoder.rs-0887 */                         let root = blob.get_root();
/* FP:decoder.rs-0888 */ 
/* FP:decoder.rs-0889 */                         let def_kind = root.tables.def_kind.get(blob, item).unwrap();
/* FP:decoder.rs-0890 */                         let def_key = root.tables.def_keys.get(blob, item).unwrap().decode(blob);
/* FP:decoder.rs-0891 */                         #[allow(rustc::symbol_intern_string_literal)]
/* FP:decoder.rs-0892 */                         let def_name = if item == CRATE_DEF_INDEX {
/* FP:decoder.rs-0893 */                             kw::Crate
/* FP:decoder.rs-0894 */                         } else {
/* FP:decoder.rs-0895 */                             def_key
/* FP:decoder.rs-0896 */                                 .disambiguated_data
/* FP:decoder.rs-0897 */                                 .data
/* FP:decoder.rs-0898 */                                 .get_opt_name()
/* FP:decoder.rs-0899 */                                 .unwrap_or_else(|| Symbol::intern("???"))
/* FP:decoder.rs-0900 */                         };
/* FP:decoder.rs-0901 */                         let visibility =
/* FP:decoder.rs-0902 */                             root.tables.visibility.get(blob, item).unwrap().decode(blob).map_id(
/* FP:decoder.rs-0903 */                                 |index| {
/* FP:decoder.rs-0904 */                                     format!(
/* FP:decoder.rs-0905 */                                         "crate{}",
/* FP:decoder.rs-0906 */                                         DefPath::make(LOCAL_CRATE, index, |parent| root
/* FP:decoder.rs-0907 */                                             .tables
/* FP:decoder.rs-0908 */                                             .def_keys
/* FP:decoder.rs-0909 */                                             .get(blob, parent)
/* FP:decoder.rs-0910 */                                             .unwrap()
/* FP:decoder.rs-0911 */                                             .decode(blob))
/* FP:decoder.rs-0912 */                                         .to_string_no_crate_verbose()
/* FP:decoder.rs-0913 */                                     )
/* FP:decoder.rs-0914 */                                 },
/* FP:decoder.rs-0915 */                             );
/* FP:decoder.rs-0916 */                         write!(
/* FP:decoder.rs-0917 */                             out,
/* FP:decoder.rs-0918 */                             "{nil: <indent$}{:?} {:?} {} {{",
/* FP:decoder.rs-0919 */                             visibility,
/* FP:decoder.rs-0920 */                             def_kind,
/* FP:decoder.rs-0921 */                             def_name,
/* FP:decoder.rs-0922 */                             nil = "",
/* FP:decoder.rs-0923 */                         )?;
/* FP:decoder.rs-0924 */ 
/* FP:decoder.rs-0925 */                         if let Some(children) =
/* FP:decoder.rs-0926 */                             root.tables.module_children_non_reexports.get(blob, item)
/* FP:decoder.rs-0927 */                         {
/* FP:decoder.rs-0928 */                             write!(out, "\n")?;
/* FP:decoder.rs-0929 */                             for child in children.decode(blob) {
/* FP:decoder.rs-0930 */                                 print_item(blob, out, child, indent + 4)?;
/* FP:decoder.rs-0931 */                             }
/* FP:decoder.rs-0932 */                             writeln!(out, "{nil: <indent$}}}", nil = "")?;
/* FP:decoder.rs-0933 */                         } else {
/* FP:decoder.rs-0934 */                             writeln!(out, "}}")?;
/* FP:decoder.rs-0935 */                         }
/* FP:decoder.rs-0936 */ 
/* FP:decoder.rs-0937 */                         Ok(())
/* FP:decoder.rs-0938 */                     }
/* FP:decoder.rs-0939 */ 
/* FP:decoder.rs-0940 */                     print_item(self, out, CRATE_DEF_INDEX, 0)?;
/* FP:decoder.rs-0941 */ 
/* FP:decoder.rs-0942 */                     write!(out, "\n")?;
/* FP:decoder.rs-0943 */                 }
/* FP:decoder.rs-0944 */ 
/* FP:decoder.rs-0945 */                 _ => {
/* FP:decoder.rs-0946 */                     writeln!(
/* FP:decoder.rs-0947 */                         out,
/* FP:decoder.rs-0948 */                         "unknown -Zls kind. allowed values are: all, root, lang_items, features, items"
/* FP:decoder.rs-0949 */                     )?;
/* FP:decoder.rs-0950 */                 }
/* FP:decoder.rs-0951 */             }
/* FP:decoder.rs-0952 */         }
/* FP:decoder.rs-0953 */ 
/* FP:decoder.rs-0954 */         Ok(())
/* FP:decoder.rs-0955 */     }
/* FP:decoder.rs-0956 */ }
/* FP:decoder.rs-0957 */ 
/* FP:decoder.rs-0958 */ impl CrateRoot {
/* FP:decoder.rs-0959 */     pub(crate) fn is_proc_macro_crate(&self) -> bool {
/* FP:decoder.rs-0960 */         self.proc_macro_data.is_some()
/* FP:decoder.rs-0961 */     }
/* FP:decoder.rs-0962 */ 
/* FP:decoder.rs-0963 */     pub(crate) fn name(&self) -> Symbol {
/* FP:decoder.rs-0964 */         self.header.name
/* FP:decoder.rs-0965 */     }
/* FP:decoder.rs-0966 */ 
/* FP:decoder.rs-0967 */     pub(crate) fn hash(&self) -> Svh {
/* FP:decoder.rs-0968 */         self.header.hash
/* FP:decoder.rs-0969 */     }
/* FP:decoder.rs-0970 */ 
/* FP:decoder.rs-0971 */     pub(crate) fn stable_crate_id(&self) -> StableCrateId {
/* FP:decoder.rs-0972 */         self.stable_crate_id
/* FP:decoder.rs-0973 */     }
/* FP:decoder.rs-0974 */ 
/* FP:decoder.rs-0975 */     pub(crate) fn decode_crate_deps<'a>(
/* FP:decoder.rs-0976 */         &self,
/* FP:decoder.rs-0977 */         metadata: &'a MetadataBlob,
/* FP:decoder.rs-0978 */     ) -> impl ExactSizeIterator<Item = CrateDep> {
/* FP:decoder.rs-0979 */         self.crate_deps.decode(metadata)
/* FP:decoder.rs-0980 */     }
/* FP:decoder.rs-0981 */ 
/* FP:decoder.rs-0982 */     pub(crate) fn decode_target_modifiers<'a>(
/* FP:decoder.rs-0983 */         &self,
/* FP:decoder.rs-0984 */         metadata: &'a MetadataBlob,
/* FP:decoder.rs-0985 */     ) -> impl ExactSizeIterator<Item = TargetModifier> {
/* FP:decoder.rs-0986 */         self.target_modifiers.decode(metadata)
/* FP:decoder.rs-0987 */     }
/* FP:decoder.rs-0988 */ }
/* FP:decoder.rs-0989 */ 
/* FP:decoder.rs-0990 */ impl<'a> CrateMetadataRef<'a> {
/* FP:decoder.rs-0991 */     fn missing(self, descr: &str, id: DefIndex) -> ! {
/* FP:decoder.rs-0992 */         bug!("missing `{descr}` for {:?}", self.local_def_id(id))
/* FP:decoder.rs-0993 */     }
/* FP:decoder.rs-0994 */ 
/* FP:decoder.rs-0995 */     fn raw_proc_macro(self, id: DefIndex) -> &'a ProcMacro {
/* FP:decoder.rs-0996 */         // DefIndex's in root.proc_macro_data have a one-to-one correspondence
/* FP:decoder.rs-0997 */         // with items in 'raw_proc_macros'.
/* FP:decoder.rs-0998 */         let pos = self
/* FP:decoder.rs-0999 */             .root
/* FP:decoder.rs-1000 */             .proc_macro_data
/* FP:decoder.rs-1001 */             .as_ref()
/* FP:decoder.rs-1002 */             .unwrap()
/* FP:decoder.rs-1003 */             .macros
/* FP:decoder.rs-1004 */             .decode(self)
/* FP:decoder.rs-1005 */             .position(|i| i == id)
/* FP:decoder.rs-1006 */             .unwrap();
/* FP:decoder.rs-1007 */         &self.raw_proc_macros.unwrap()[pos]
/* FP:decoder.rs-1008 */     }
/* FP:decoder.rs-1009 */ 
/* FP:decoder.rs-1010 */     fn opt_item_name(self, item_index: DefIndex) -> Option<Symbol> {
/* FP:decoder.rs-1011 */         let def_key = self.def_key(item_index);
/* FP:decoder.rs-1012 */         def_key.disambiguated_data.data.get_opt_name().or_else(|| {
/* FP:decoder.rs-1013 */             if def_key.disambiguated_data.data == DefPathData::Ctor {
/* FP:decoder.rs-1014 */                 let parent_index = def_key.parent.expect("no parent for a constructor");
/* FP:decoder.rs-1015 */                 self.def_key(parent_index).disambiguated_data.data.get_opt_name()
/* FP:decoder.rs-1016 */             } else {
/* FP:decoder.rs-1017 */                 None
/* FP:decoder.rs-1018 */             }
/* FP:decoder.rs-1019 */         })
/* FP:decoder.rs-1020 */     }
/* FP:decoder.rs-1021 */ 
/* FP:decoder.rs-1022 */     fn item_name(self, item_index: DefIndex) -> Symbol {
/* FP:decoder.rs-1023 */         self.opt_item_name(item_index).expect("no encoded ident for item")
/* FP:decoder.rs-1024 */     }
/* FP:decoder.rs-1025 */ 
/* FP:decoder.rs-1026 */     fn opt_item_ident(self, item_index: DefIndex, sess: &Session) -> Option<Ident> {
/* FP:decoder.rs-1027 */         let name = self.opt_item_name(item_index)?;
/* FP:decoder.rs-1028 */         let span = self
/* FP:decoder.rs-1029 */             .root
/* FP:decoder.rs-1030 */             .tables
/* FP:decoder.rs-1031 */             .def_ident_span
/* FP:decoder.rs-1032 */             .get(self, item_index)
/* FP:decoder.rs-1033 */             .unwrap_or_else(|| self.missing("def_ident_span", item_index))
/* FP:decoder.rs-1034 */             .decode((self, sess));
/* FP:decoder.rs-1035 */         Some(Ident::new(name, span))
/* FP:decoder.rs-1036 */     }
/* FP:decoder.rs-1037 */ 
/* FP:decoder.rs-1038 */     fn item_ident(self, item_index: DefIndex, sess: &Session) -> Ident {
/* FP:decoder.rs-1039 */         self.opt_item_ident(item_index, sess).expect("no encoded ident for item")
/* FP:decoder.rs-1040 */     }
/* FP:decoder.rs-1041 */ 
/* FP:decoder.rs-1042 */     #[inline]
/* FP:decoder.rs-1043 */     pub(super) fn map_encoded_cnum_to_current(self, cnum: CrateNum) -> CrateNum {
/* FP:decoder.rs-1044 */         if cnum == LOCAL_CRATE { self.cnum } else { self.cnum_map[cnum] }
/* FP:decoder.rs-1045 */     }
/* FP:decoder.rs-1046 */ 
/* FP:decoder.rs-1047 */     fn def_kind(self, item_id: DefIndex) -> DefKind {
/* FP:decoder.rs-1048 */         self.root
/* FP:decoder.rs-1049 */             .tables
/* FP:decoder.rs-1050 */             .def_kind
/* FP:decoder.rs-1051 */             .get(self, item_id)
/* FP:decoder.rs-1052 */             .unwrap_or_else(|| self.missing("def_kind", item_id))
/* FP:decoder.rs-1053 */     }
/* FP:decoder.rs-1054 */ 
/* FP:decoder.rs-1055 */     fn get_span(self, index: DefIndex, sess: &Session) -> Span {
/* FP:decoder.rs-1056 */         self.root
/* FP:decoder.rs-1057 */             .tables
/* FP:decoder.rs-1058 */             .def_span
/* FP:decoder.rs-1059 */             .get(self, index)
/* FP:decoder.rs-1060 */             .unwrap_or_else(|| self.missing("def_span", index))
/* FP:decoder.rs-1061 */             .decode((self, sess))
/* FP:decoder.rs-1062 */     }
/* FP:decoder.rs-1063 */ 
/* FP:decoder.rs-1064 */     fn load_proc_macro<'tcx>(self, id: DefIndex, tcx: TyCtxt<'tcx>) -> SyntaxExtension {
/* FP:decoder.rs-1065 */         let (name, kind, helper_attrs) = match *self.raw_proc_macro(id) {
/* FP:decoder.rs-1066 */             ProcMacro::CustomDerive { trait_name, attributes, client } => {
/* FP:decoder.rs-1067 */                 let helper_attrs =
/* FP:decoder.rs-1068 */                     attributes.iter().cloned().map(Symbol::intern).collect::<Vec<_>>();
/* FP:decoder.rs-1069 */                 (
/* FP:decoder.rs-1070 */                     trait_name,
/* FP:decoder.rs-1071 */                     SyntaxExtensionKind::Derive(Arc::new(DeriveProcMacro { client })),
/* FP:decoder.rs-1072 */                     helper_attrs,
/* FP:decoder.rs-1073 */                 )
/* FP:decoder.rs-1074 */             }
/* FP:decoder.rs-1075 */             ProcMacro::Attr { name, client } => {
/* FP:decoder.rs-1076 */                 (name, SyntaxExtensionKind::Attr(Arc::new(AttrProcMacro { client })), Vec::new())
/* FP:decoder.rs-1077 */             }
/* FP:decoder.rs-1078 */             ProcMacro::Bang { name, client } => {
/* FP:decoder.rs-1079 */                 (name, SyntaxExtensionKind::Bang(Arc::new(BangProcMacro { client })), Vec::new())
/* FP:decoder.rs-1080 */             }
/* FP:decoder.rs-1081 */         };
/* FP:decoder.rs-1082 */ 
/* FP:decoder.rs-1083 */         let sess = tcx.sess;
/* FP:decoder.rs-1084 */         let attrs: Vec<_> = self.get_item_attrs(id, sess).collect();
/* FP:decoder.rs-1085 */         SyntaxExtension::new(
/* FP:decoder.rs-1086 */             sess,
/* FP:decoder.rs-1087 */             kind,
/* FP:decoder.rs-1088 */             self.get_span(id, sess),
/* FP:decoder.rs-1089 */             helper_attrs,
/* FP:decoder.rs-1090 */             self.root.edition,
/* FP:decoder.rs-1091 */             Symbol::intern(name),
/* FP:decoder.rs-1092 */             &attrs,
/* FP:decoder.rs-1093 */             false,
/* FP:decoder.rs-1094 */         )
/* FP:decoder.rs-1095 */     }
/* FP:decoder.rs-1096 */ 
/* FP:decoder.rs-1097 */     fn get_variant(
/* FP:decoder.rs-1098 */         self,
/* FP:decoder.rs-1099 */         kind: DefKind,
/* FP:decoder.rs-1100 */         index: DefIndex,
/* FP:decoder.rs-1101 */         parent_did: DefId,
/* FP:decoder.rs-1102 */     ) -> (VariantIdx, ty::VariantDef) {
/* FP:decoder.rs-1103 */         let adt_kind = match kind {
/* FP:decoder.rs-1104 */             DefKind::Variant => ty::AdtKind::Enum,
/* FP:decoder.rs-1105 */             DefKind::Struct => ty::AdtKind::Struct,
/* FP:decoder.rs-1106 */             DefKind::Union => ty::AdtKind::Union,
/* FP:decoder.rs-1107 */             _ => bug!(),
/* FP:decoder.rs-1108 */         };
/* FP:decoder.rs-1109 */ 
/* FP:decoder.rs-1110 */         let data = self.root.tables.variant_data.get(self, index).unwrap().decode(self);
/* FP:decoder.rs-1111 */ 
/* FP:decoder.rs-1112 */         let variant_did =
/* FP:decoder.rs-1113 */             if adt_kind == ty::AdtKind::Enum { Some(self.local_def_id(index)) } else { None };
/* FP:decoder.rs-1114 */         let ctor = data.ctor.map(|(kind, index)| (kind, self.local_def_id(index)));
/* FP:decoder.rs-1115 */ 
/* FP:decoder.rs-1116 */         (
/* FP:decoder.rs-1117 */             data.idx,
/* FP:decoder.rs-1118 */             ty::VariantDef::new(
/* FP:decoder.rs-1119 */                 self.item_name(index),
/* FP:decoder.rs-1120 */                 variant_did,
/* FP:decoder.rs-1121 */                 ctor,
/* FP:decoder.rs-1122 */                 data.discr,
/* FP:decoder.rs-1123 */                 self.get_associated_item_or_field_def_ids(index)
/* FP:decoder.rs-1124 */                     .map(|did| ty::FieldDef {
/* FP:decoder.rs-1125 */                         did,
/* FP:decoder.rs-1126 */                         name: self.item_name(did.index),
/* FP:decoder.rs-1127 */                         vis: self.get_visibility(did.index),
/* FP:decoder.rs-1128 */                         safety: self.get_safety(did.index),
/* FP:decoder.rs-1129 */                         value: self.get_default_field(did.index),
/* FP:decoder.rs-1130 */                     })
/* FP:decoder.rs-1131 */                     .collect(),
/* FP:decoder.rs-1132 */                 parent_did,
/* FP:decoder.rs-1133 */                 None,
/* FP:decoder.rs-1134 */                 data.is_non_exhaustive,
/* FP:decoder.rs-1135 */             ),
/* FP:decoder.rs-1136 */         )
/* FP:decoder.rs-1137 */     }
/* FP:decoder.rs-1138 */ 
/* FP:decoder.rs-1139 */     fn get_adt_def<'tcx>(self, item_id: DefIndex, tcx: TyCtxt<'tcx>) -> ty::AdtDef<'tcx> {
/* FP:decoder.rs-1140 */         let kind = self.def_kind(item_id);
/* FP:decoder.rs-1141 */         let did = self.local_def_id(item_id);
/* FP:decoder.rs-1142 */ 
/* FP:decoder.rs-1143 */         let adt_kind = match kind {
/* FP:decoder.rs-1144 */             DefKind::Enum => ty::AdtKind::Enum,
/* FP:decoder.rs-1145 */             DefKind::Struct => ty::AdtKind::Struct,
/* FP:decoder.rs-1146 */             DefKind::Union => ty::AdtKind::Union,
/* FP:decoder.rs-1147 */             _ => bug!("get_adt_def called on a non-ADT {:?}", did),
/* FP:decoder.rs-1148 */         };
/* FP:decoder.rs-1149 */         let repr = self.root.tables.repr_options.get(self, item_id).unwrap().decode(self);
/* FP:decoder.rs-1150 */ 
/* FP:decoder.rs-1151 */         let mut variants: Vec<_> = if let ty::AdtKind::Enum = adt_kind {
/* FP:decoder.rs-1152 */             self.root
/* FP:decoder.rs-1153 */                 .tables
/* FP:decoder.rs-1154 */                 .module_children_non_reexports
/* FP:decoder.rs-1155 */                 .get(self, item_id)
/* FP:decoder.rs-1156 */                 .expect("variants are not encoded for an enum")
/* FP:decoder.rs-1157 */                 .decode(self)
/* FP:decoder.rs-1158 */                 .filter_map(|index| {
/* FP:decoder.rs-1159 */                     let kind = self.def_kind(index);
/* FP:decoder.rs-1160 */                     match kind {
/* FP:decoder.rs-1161 */                         DefKind::Ctor(..) => None,
/* FP:decoder.rs-1162 */                         _ => Some(self.get_variant(kind, index, did)),
/* FP:decoder.rs-1163 */                     }
/* FP:decoder.rs-1164 */                 })
/* FP:decoder.rs-1165 */                 .collect()
/* FP:decoder.rs-1166 */         } else {
/* FP:decoder.rs-1167 */             std::iter::once(self.get_variant(kind, item_id, did)).collect()
/* FP:decoder.rs-1168 */         };
/* FP:decoder.rs-1169 */ 
/* FP:decoder.rs-1170 */         variants.sort_by_key(|(idx, _)| *idx);
/* FP:decoder.rs-1171 */ 
/* FP:decoder.rs-1172 */         tcx.mk_adt_def(
/* FP:decoder.rs-1173 */             did,
/* FP:decoder.rs-1174 */             adt_kind,
/* FP:decoder.rs-1175 */             variants.into_iter().map(|(_, variant)| variant).collect(),
/* FP:decoder.rs-1176 */             repr,
/* FP:decoder.rs-1177 */         )
/* FP:decoder.rs-1178 */     }
/* FP:decoder.rs-1179 */ 
/* FP:decoder.rs-1180 */     fn get_visibility(self, id: DefIndex) -> Visibility<DefId> {
/* FP:decoder.rs-1181 */         self.root
/* FP:decoder.rs-1182 */             .tables
/* FP:decoder.rs-1183 */             .visibility
/* FP:decoder.rs-1184 */             .get(self, id)
/* FP:decoder.rs-1185 */             .unwrap_or_else(|| self.missing("visibility", id))
/* FP:decoder.rs-1186 */             .decode(self)
/* FP:decoder.rs-1187 */             .map_id(|index| self.local_def_id(index))
/* FP:decoder.rs-1188 */     }
/* FP:decoder.rs-1189 */ 
/* FP:decoder.rs-1190 */     fn get_safety(self, id: DefIndex) -> Safety {
/* FP:decoder.rs-1191 */         self.root.tables.safety.get(self, id).unwrap_or_else(|| self.missing("safety", id))
/* FP:decoder.rs-1192 */     }
/* FP:decoder.rs-1193 */ 
/* FP:decoder.rs-1194 */     fn get_default_field(self, id: DefIndex) -> Option<DefId> {
/* FP:decoder.rs-1195 */         self.root.tables.default_fields.get(self, id).map(|d| d.decode(self))
/* FP:decoder.rs-1196 */     }
/* FP:decoder.rs-1197 */ 
/* FP:decoder.rs-1198 */     fn get_expn_that_defined(self, id: DefIndex, sess: &Session) -> ExpnId {
/* FP:decoder.rs-1199 */         self.root
/* FP:decoder.rs-1200 */             .tables
/* FP:decoder.rs-1201 */             .expn_that_defined
/* FP:decoder.rs-1202 */             .get(self, id)
/* FP:decoder.rs-1203 */             .unwrap_or_else(|| self.missing("expn_that_defined", id))
/* FP:decoder.rs-1204 */             .decode((self, sess))
/* FP:decoder.rs-1205 */     }
/* FP:decoder.rs-1206 */ 
/* FP:decoder.rs-1207 */     fn get_debugger_visualizers(self) -> Vec<DebuggerVisualizerFile> {
/* FP:decoder.rs-1208 */         self.root.debugger_visualizers.decode(self).collect::<Vec<_>>()
/* FP:decoder.rs-1209 */     }
/* FP:decoder.rs-1210 */ 
/* FP:decoder.rs-1211 */     /// Iterates over all the stability attributes in the given crate.
/* FP:decoder.rs-1212 */     fn get_lib_features(self) -> LibFeatures {
/* FP:decoder.rs-1213 */         LibFeatures {
/* FP:decoder.rs-1214 */             stability: self
/* FP:decoder.rs-1215 */                 .root
/* FP:decoder.rs-1216 */                 .lib_features
/* FP:decoder.rs-1217 */                 .decode(self)
/* FP:decoder.rs-1218 */                 .map(|(sym, stab)| (sym, (stab, DUMMY_SP)))
/* FP:decoder.rs-1219 */                 .collect(),
/* FP:decoder.rs-1220 */         }
/* FP:decoder.rs-1221 */     }
/* FP:decoder.rs-1222 */ 
/* FP:decoder.rs-1223 */     /// Iterates over the stability implications in the given crate (when a `#[unstable]` attribute
/* FP:decoder.rs-1224 */     /// has an `implied_by` meta item, then the mapping from the implied feature to the actual
/* FP:decoder.rs-1225 */     /// feature is a stability implication).
/* FP:decoder.rs-1226 */     fn get_stability_implications<'tcx>(self, tcx: TyCtxt<'tcx>) -> &'tcx [(Symbol, Symbol)] {
/* FP:decoder.rs-1227 */         tcx.arena.alloc_from_iter(self.root.stability_implications.decode(self))
/* FP:decoder.rs-1228 */     }
/* FP:decoder.rs-1229 */ 
/* FP:decoder.rs-1230 */     /// Iterates over the lang items in the given crate.
/* FP:decoder.rs-1231 */     fn get_lang_items<'tcx>(self, tcx: TyCtxt<'tcx>) -> &'tcx [(DefId, LangItem)] {
/* FP:decoder.rs-1232 */         tcx.arena.alloc_from_iter(
/* FP:decoder.rs-1233 */             self.root
/* FP:decoder.rs-1234 */                 .lang_items
/* FP:decoder.rs-1235 */                 .decode(self)
/* FP:decoder.rs-1236 */                 .map(move |(def_index, index)| (self.local_def_id(def_index), index)),
/* FP:decoder.rs-1237 */         )
/* FP:decoder.rs-1238 */     }
/* FP:decoder.rs-1239 */ 
/* FP:decoder.rs-1240 */     fn get_stripped_cfg_items<'tcx>(
/* FP:decoder.rs-1241 */         self,
/* FP:decoder.rs-1242 */         cnum: CrateNum,
/* FP:decoder.rs-1243 */         tcx: TyCtxt<'tcx>,
/* FP:decoder.rs-1244 */     ) -> &'tcx [StrippedCfgItem] {
/* FP:decoder.rs-1245 */         let item_names = self
/* FP:decoder.rs-1246 */             .root
/* FP:decoder.rs-1247 */             .stripped_cfg_items
/* FP:decoder.rs-1248 */             .decode((self, tcx))
/* FP:decoder.rs-1249 */             .map(|item| item.map_mod_id(|index| DefId { krate: cnum, index }));
/* FP:decoder.rs-1250 */         tcx.arena.alloc_from_iter(item_names)
/* FP:decoder.rs-1251 */     }
/* FP:decoder.rs-1252 */ 
/* FP:decoder.rs-1253 */     /// Iterates over the diagnostic items in the given crate.
/* FP:decoder.rs-1254 */     fn get_diagnostic_items(self) -> DiagnosticItems {
/* FP:decoder.rs-1255 */         let mut id_to_name = DefIdMap::default();
/* FP:decoder.rs-1256 */         let name_to_id = self
/* FP:decoder.rs-1257 */             .root
/* FP:decoder.rs-1258 */             .diagnostic_items
/* FP:decoder.rs-1259 */             .decode(self)
/* FP:decoder.rs-1260 */             .map(|(name, def_index)| {
/* FP:decoder.rs-1261 */                 let id = self.local_def_id(def_index);
/* FP:decoder.rs-1262 */                 id_to_name.insert(id, name);
/* FP:decoder.rs-1263 */                 (name, id)
/* FP:decoder.rs-1264 */             })
/* FP:decoder.rs-1265 */             .collect();
/* FP:decoder.rs-1266 */         DiagnosticItems { id_to_name, name_to_id }
/* FP:decoder.rs-1267 */     }
/* FP:decoder.rs-1268 */ 
/* FP:decoder.rs-1269 */     fn get_mod_child(self, id: DefIndex, sess: &Session) -> ModChild {
/* FP:decoder.rs-1270 */         let ident = self.item_ident(id, sess);
/* FP:decoder.rs-1271 */         let res = Res::Def(self.def_kind(id), self.local_def_id(id));
/* FP:decoder.rs-1272 */         let vis = self.get_visibility(id);
/* FP:decoder.rs-1273 */ 
/* FP:decoder.rs-1274 */         ModChild { ident, res, vis, reexport_chain: Default::default() }
/* FP:decoder.rs-1275 */     }
/* FP:decoder.rs-1276 */ 
/* FP:decoder.rs-1277 */     /// Iterates over all named children of the given module,
/* FP:decoder.rs-1278 */     /// including both proper items and reexports.
/* FP:decoder.rs-1279 */     /// Module here is understood in name resolution sense - it can be a `mod` item,
/* FP:decoder.rs-1280 */     /// or a crate root, or an enum, or a trait.
/* FP:decoder.rs-1281 */     fn get_module_children(
/* FP:decoder.rs-1282 */         self,
/* FP:decoder.rs-1283 */         id: DefIndex,
/* FP:decoder.rs-1284 */         sess: &'a Session,
/* FP:decoder.rs-1285 */     ) -> impl Iterator<Item = ModChild> {
/* FP:decoder.rs-1286 */         gen move {
/* FP:decoder.rs-1287 */             if let Some(data) = &self.root.proc_macro_data {
/* FP:decoder.rs-1288 */                 // If we are loading as a proc macro, we want to return
/* FP:decoder.rs-1289 */                 // the view of this crate as a proc macro crate.
/* FP:decoder.rs-1290 */                 if id == CRATE_DEF_INDEX {
/* FP:decoder.rs-1291 */                     for child_index in data.macros.decode(self) {
/* FP:decoder.rs-1292 */                         yield self.get_mod_child(child_index, sess);
/* FP:decoder.rs-1293 */                     }
/* FP:decoder.rs-1294 */                 }
/* FP:decoder.rs-1295 */             } else {
/* FP:decoder.rs-1296 */                 // Iterate over all children.
/* FP:decoder.rs-1297 */                 let non_reexports = self.root.tables.module_children_non_reexports.get(self, id);
/* FP:decoder.rs-1298 */                 for child_index in non_reexports.unwrap().decode(self) {
/* FP:decoder.rs-1299 */                     yield self.get_mod_child(child_index, sess);
/* FP:decoder.rs-1300 */                 }
/* FP:decoder.rs-1301 */ 
/* FP:decoder.rs-1302 */                 let reexports = self.root.tables.module_children_reexports.get(self, id);
/* FP:decoder.rs-1303 */                 if !reexports.is_default() {
/* FP:decoder.rs-1304 */                     for reexport in reexports.decode((self, sess)) {
/* FP:decoder.rs-1305 */                         yield reexport;
/* FP:decoder.rs-1306 */                     }
/* FP:decoder.rs-1307 */                 }
/* FP:decoder.rs-1308 */             }
/* FP:decoder.rs-1309 */         }
/* FP:decoder.rs-1310 */     }
/* FP:decoder.rs-1311 */ 
/* FP:decoder.rs-1312 */     fn is_ctfe_mir_available(self, id: DefIndex) -> bool {
/* FP:decoder.rs-1313 */         self.root.tables.mir_for_ctfe.get(self, id).is_some()
/* FP:decoder.rs-1314 */     }
/* FP:decoder.rs-1315 */ 
/* FP:decoder.rs-1316 */     fn is_item_mir_available(self, id: DefIndex) -> bool {
/* FP:decoder.rs-1317 */         self.root.tables.optimized_mir.get(self, id).is_some()
/* FP:decoder.rs-1318 */     }
/* FP:decoder.rs-1319 */ 
/* FP:decoder.rs-1320 */     fn get_fn_has_self_parameter(self, id: DefIndex, sess: &'a Session) -> bool {
/* FP:decoder.rs-1321 */         self.root
/* FP:decoder.rs-1322 */             .tables
/* FP:decoder.rs-1323 */             .fn_arg_idents
/* FP:decoder.rs-1324 */             .get(self, id)
/* FP:decoder.rs-1325 */             .expect("argument names not encoded for a function")
/* FP:decoder.rs-1326 */             .decode((self, sess))
/* FP:decoder.rs-1327 */             .nth(0)
/* FP:decoder.rs-1328 */             .is_some_and(|ident| matches!(ident, Some(Ident { name: kw::SelfLower, .. })))
/* FP:decoder.rs-1329 */     }
/* FP:decoder.rs-1330 */ 
/* FP:decoder.rs-1331 */     fn get_associated_item_or_field_def_ids(self, id: DefIndex) -> impl Iterator<Item = DefId> {
/* FP:decoder.rs-1332 */         self.root
/* FP:decoder.rs-1333 */             .tables
/* FP:decoder.rs-1334 */             .associated_item_or_field_def_ids
/* FP:decoder.rs-1335 */             .get(self, id)
/* FP:decoder.rs-1336 */             .unwrap_or_else(|| self.missing("associated_item_or_field_def_ids", id))
/* FP:decoder.rs-1337 */             .decode(self)
/* FP:decoder.rs-1338 */             .map(move |child_index| self.local_def_id(child_index))
/* FP:decoder.rs-1339 */     }
/* FP:decoder.rs-1340 */ 
/* FP:decoder.rs-1341 */     fn get_associated_item(self, id: DefIndex, sess: &'a Session) -> ty::AssocItem {
/* FP:decoder.rs-1342 */         let kind = match self.def_kind(id) {
/* FP:decoder.rs-1343 */             DefKind::AssocConst => ty::AssocKind::Const { name: self.item_name(id) },
/* FP:decoder.rs-1344 */             DefKind::AssocFn => ty::AssocKind::Fn {
/* FP:decoder.rs-1345 */                 name: self.item_name(id),
/* FP:decoder.rs-1346 */                 has_self: self.get_fn_has_self_parameter(id, sess),
/* FP:decoder.rs-1347 */             },
/* FP:decoder.rs-1348 */             DefKind::AssocTy => {
/* FP:decoder.rs-1349 */                 let data = if let Some(rpitit_info) = self.root.tables.opt_rpitit_info.get(self, id)
/* FP:decoder.rs-1350 */                 {
/* FP:decoder.rs-1351 */                     ty::AssocTypeData::Rpitit(rpitit_info.decode(self))
/* FP:decoder.rs-1352 */                 } else {
/* FP:decoder.rs-1353 */                     ty::AssocTypeData::Normal(self.item_name(id))
/* FP:decoder.rs-1354 */                 };
/* FP:decoder.rs-1355 */                 ty::AssocKind::Type { data }
/* FP:decoder.rs-1356 */             }
/* FP:decoder.rs-1357 */             _ => bug!("cannot get associated-item of `{:?}`", self.def_key(id)),
/* FP:decoder.rs-1358 */         };
/* FP:decoder.rs-1359 */         let container = self.root.tables.assoc_container.get(self, id).unwrap().decode(self);
/* FP:decoder.rs-1360 */ 
/* FP:decoder.rs-1361 */         ty::AssocItem { kind, def_id: self.local_def_id(id), container }
/* FP:decoder.rs-1362 */     }
/* FP:decoder.rs-1363 */ 
/* FP:decoder.rs-1364 */     fn get_ctor(self, node_id: DefIndex) -> Option<(CtorKind, DefId)> {
/* FP:decoder.rs-1365 */         match self.def_kind(node_id) {
/* FP:decoder.rs-1366 */             DefKind::Struct | DefKind::Variant => {
/* FP:decoder.rs-1367 */                 let vdata = self.root.tables.variant_data.get(self, node_id).unwrap().decode(self);
/* FP:decoder.rs-1368 */                 vdata.ctor.map(|(kind, index)| (kind, self.local_def_id(index)))
/* FP:decoder.rs-1369 */             }
/* FP:decoder.rs-1370 */             _ => None,
/* FP:decoder.rs-1371 */         }
/* FP:decoder.rs-1372 */     }
/* FP:decoder.rs-1373 */ 
/* FP:decoder.rs-1374 */     fn get_item_attrs(
/* FP:decoder.rs-1375 */         self,
/* FP:decoder.rs-1376 */         id: DefIndex,
/* FP:decoder.rs-1377 */         sess: &'a Session,
/* FP:decoder.rs-1378 */     ) -> impl Iterator<Item = hir::Attribute> {
/* FP:decoder.rs-1379 */         self.root
/* FP:decoder.rs-1380 */             .tables
/* FP:decoder.rs-1381 */             .attributes
/* FP:decoder.rs-1382 */             .get(self, id)
/* FP:decoder.rs-1383 */             .unwrap_or_else(|| {
/* FP:decoder.rs-1384 */                 // Structure and variant constructors don't have any attributes encoded for them,
/* FP:decoder.rs-1385 */                 // but we assume that someone passing a constructor ID actually wants to look at
/* FP:decoder.rs-1386 */                 // the attributes on the corresponding struct or variant.
/* FP:decoder.rs-1387 */                 let def_key = self.def_key(id);
/* FP:decoder.rs-1388 */                 assert_eq!(def_key.disambiguated_data.data, DefPathData::Ctor);
/* FP:decoder.rs-1389 */                 let parent_id = def_key.parent.expect("no parent for a constructor");
/* FP:decoder.rs-1390 */                 self.root
/* FP:decoder.rs-1391 */                     .tables
/* FP:decoder.rs-1392 */                     .attributes
/* FP:decoder.rs-1393 */                     .get(self, parent_id)
/* FP:decoder.rs-1394 */                     .expect("no encoded attributes for a structure or variant")
/* FP:decoder.rs-1395 */             })
/* FP:decoder.rs-1396 */             .decode((self, sess))
/* FP:decoder.rs-1397 */     }
/* FP:decoder.rs-1398 */ 
/* FP:decoder.rs-1399 */     fn get_inherent_implementations_for_type<'tcx>(
/* FP:decoder.rs-1400 */         self,
/* FP:decoder.rs-1401 */         tcx: TyCtxt<'tcx>,
/* FP:decoder.rs-1402 */         id: DefIndex,
/* FP:decoder.rs-1403 */     ) -> &'tcx [DefId] {
/* FP:decoder.rs-1404 */         tcx.arena.alloc_from_iter(
/* FP:decoder.rs-1405 */             self.root
/* FP:decoder.rs-1406 */                 .tables
/* FP:decoder.rs-1407 */                 .inherent_impls
/* FP:decoder.rs-1408 */                 .get(self, id)
/* FP:decoder.rs-1409 */                 .decode(self)
/* FP:decoder.rs-1410 */                 .map(|index| self.local_def_id(index)),
/* FP:decoder.rs-1411 */         )
/* FP:decoder.rs-1412 */     }
/* FP:decoder.rs-1413 */ 
/* FP:decoder.rs-1414 */     /// Decodes all traits in the crate (for rustdoc and rustc diagnostics).
/* FP:decoder.rs-1415 */     fn get_traits(self) -> impl Iterator<Item = DefId> {
/* FP:decoder.rs-1416 */         self.root.traits.decode(self).map(move |index| self.local_def_id(index))
/* FP:decoder.rs-1417 */     }
/* FP:decoder.rs-1418 */ 
/* FP:decoder.rs-1419 */     /// Decodes all trait impls in the crate (for rustdoc).
/* FP:decoder.rs-1420 */     fn get_trait_impls(self) -> impl Iterator<Item = DefId> {
/* FP:decoder.rs-1421 */         self.cdata.trait_impls.values().flat_map(move |impls| {
/* FP:decoder.rs-1422 */             impls.decode(self).map(move |(impl_index, _)| self.local_def_id(impl_index))
/* FP:decoder.rs-1423 */         })
/* FP:decoder.rs-1424 */     }
/* FP:decoder.rs-1425 */ 
/* FP:decoder.rs-1426 */     fn get_incoherent_impls<'tcx>(self, tcx: TyCtxt<'tcx>, simp: SimplifiedType) -> &'tcx [DefId] {
/* FP:decoder.rs-1427 */         if let Some(impls) = self.cdata.incoherent_impls.get(&simp) {
/* FP:decoder.rs-1428 */             tcx.arena.alloc_from_iter(impls.decode(self).map(|idx| self.local_def_id(idx)))
/* FP:decoder.rs-1429 */         } else {
/* FP:decoder.rs-1430 */             &[]
/* FP:decoder.rs-1431 */         }
/* FP:decoder.rs-1432 */     }
/* FP:decoder.rs-1433 */ 
/* FP:decoder.rs-1434 */     fn get_implementations_of_trait<'tcx>(
/* FP:decoder.rs-1435 */         self,
/* FP:decoder.rs-1436 */         tcx: TyCtxt<'tcx>,
/* FP:decoder.rs-1437 */         trait_def_id: DefId,
/* FP:decoder.rs-1438 */     ) -> &'tcx [(DefId, Option<SimplifiedType>)] {
/* FP:decoder.rs-1439 */         if self.trait_impls.is_empty() {
/* FP:decoder.rs-1440 */             return &[];
/* FP:decoder.rs-1441 */         }
/* FP:decoder.rs-1442 */ 
/* FP:decoder.rs-1443 */         // Do a reverse lookup beforehand to avoid touching the crate_num
/* FP:decoder.rs-1444 */         // hash map in the loop below.
/* FP:decoder.rs-1445 */         let key = match self.reverse_translate_def_id(trait_def_id) {
/* FP:decoder.rs-1446 */             Some(def_id) => (def_id.krate.as_u32(), def_id.index),
/* FP:decoder.rs-1447 */             None => return &[],
/* FP:decoder.rs-1448 */         };
/* FP:decoder.rs-1449 */ 
/* FP:decoder.rs-1450 */         if let Some(impls) = self.trait_impls.get(&key) {
/* FP:decoder.rs-1451 */             tcx.arena.alloc_from_iter(
/* FP:decoder.rs-1452 */                 impls
/* FP:decoder.rs-1453 */                     .decode(self)
/* FP:decoder.rs-1454 */                     .map(|(idx, simplified_self_ty)| (self.local_def_id(idx), simplified_self_ty)),
/* FP:decoder.rs-1455 */             )
/* FP:decoder.rs-1456 */         } else {
/* FP:decoder.rs-1457 */             &[]
/* FP:decoder.rs-1458 */         }
/* FP:decoder.rs-1459 */     }
/* FP:decoder.rs-1460 */ 
/* FP:decoder.rs-1461 */     fn get_native_libraries(self, sess: &'a Session) -> impl Iterator<Item = NativeLib> {
/* FP:decoder.rs-1462 */         self.root.native_libraries.decode((self, sess))
/* FP:decoder.rs-1463 */     }
/* FP:decoder.rs-1464 */ 
/* FP:decoder.rs-1465 */     fn get_proc_macro_quoted_span(self, index: usize, sess: &Session) -> Span {
/* FP:decoder.rs-1466 */         self.root
/* FP:decoder.rs-1467 */             .tables
/* FP:decoder.rs-1468 */             .proc_macro_quoted_spans
/* FP:decoder.rs-1469 */             .get(self, index)
/* FP:decoder.rs-1470 */             .unwrap_or_else(|| panic!("Missing proc macro quoted span: {index:?}"))
/* FP:decoder.rs-1471 */             .decode((self, sess))
/* FP:decoder.rs-1472 */     }
/* FP:decoder.rs-1473 */ 
/* FP:decoder.rs-1474 */     fn get_foreign_modules(self, sess: &'a Session) -> impl Iterator<Item = ForeignModule> {
/* FP:decoder.rs-1475 */         self.root.foreign_modules.decode((self, sess))
/* FP:decoder.rs-1476 */     }
/* FP:decoder.rs-1477 */ 
/* FP:decoder.rs-1478 */     fn get_dylib_dependency_formats<'tcx>(
/* FP:decoder.rs-1479 */         self,
/* FP:decoder.rs-1480 */         tcx: TyCtxt<'tcx>,
/* FP:decoder.rs-1481 */     ) -> &'tcx [(CrateNum, LinkagePreference)] {
/* FP:decoder.rs-1482 */         tcx.arena.alloc_from_iter(
/* FP:decoder.rs-1483 */             self.root.dylib_dependency_formats.decode(self).enumerate().flat_map(|(i, link)| {
/* FP:decoder.rs-1484 */                 let cnum = CrateNum::new(i + 1); // We skipped LOCAL_CRATE when encoding
/* FP:decoder.rs-1485 */                 link.map(|link| (self.cnum_map[cnum], link))
/* FP:decoder.rs-1486 */             }),
/* FP:decoder.rs-1487 */         )
/* FP:decoder.rs-1488 */     }
/* FP:decoder.rs-1489 */ 
/* FP:decoder.rs-1490 */     fn get_missing_lang_items<'tcx>(self, tcx: TyCtxt<'tcx>) -> &'tcx [LangItem] {
/* FP:decoder.rs-1491 */         tcx.arena.alloc_from_iter(self.root.lang_items_missing.decode(self))
/* FP:decoder.rs-1492 */     }
/* FP:decoder.rs-1493 */ 
/* FP:decoder.rs-1494 */     fn get_exportable_items(self) -> impl Iterator<Item = DefId> {
/* FP:decoder.rs-1495 */         self.root.exportable_items.decode(self).map(move |index| self.local_def_id(index))
/* FP:decoder.rs-1496 */     }
/* FP:decoder.rs-1497 */ 
/* FP:decoder.rs-1498 */     fn get_stable_order_of_exportable_impls(self) -> impl Iterator<Item = (DefId, usize)> {
/* FP:decoder.rs-1499 */         self.root
/* FP:decoder.rs-1500 */             .stable_order_of_exportable_impls
/* FP:decoder.rs-1501 */             .decode(self)
/* FP:decoder.rs-1502 */             .map(move |v| (self.local_def_id(v.0), v.1))
/* FP:decoder.rs-1503 */     }
/* FP:decoder.rs-1504 */ 
/* FP:decoder.rs-1505 */     fn exported_non_generic_symbols<'tcx>(
/* FP:decoder.rs-1506 */         self,
/* FP:decoder.rs-1507 */         tcx: TyCtxt<'tcx>,
/* FP:decoder.rs-1508 */     ) -> &'tcx [(ExportedSymbol<'tcx>, SymbolExportInfo)] {
/* FP:decoder.rs-1509 */         tcx.arena.alloc_from_iter(self.root.exported_non_generic_symbols.decode((self, tcx)))
/* FP:decoder.rs-1510 */     }
/* FP:decoder.rs-1511 */ 
/* FP:decoder.rs-1512 */     fn exported_generic_symbols<'tcx>(
/* FP:decoder.rs-1513 */         self,
/* FP:decoder.rs-1514 */         tcx: TyCtxt<'tcx>,
/* FP:decoder.rs-1515 */     ) -> &'tcx [(ExportedSymbol<'tcx>, SymbolExportInfo)] {
/* FP:decoder.rs-1516 */         tcx.arena.alloc_from_iter(self.root.exported_generic_symbols.decode((self, tcx)))
/* FP:decoder.rs-1517 */     }
/* FP:decoder.rs-1518 */ 
/* FP:decoder.rs-1519 */     fn get_macro(self, id: DefIndex, sess: &Session) -> ast::MacroDef {
/* FP:decoder.rs-1520 */         match self.def_kind(id) {
/* FP:decoder.rs-1521 */             DefKind::Macro(_) => {
/* FP:decoder.rs-1522 */                 let macro_rules = self.root.tables.is_macro_rules.get(self, id);
/* FP:decoder.rs-1523 */                 let body =
/* FP:decoder.rs-1524 */                     self.root.tables.macro_definition.get(self, id).unwrap().decode((self, sess));
/* FP:decoder.rs-1525 */                 ast::MacroDef { macro_rules, body: Box::new(body) }
/* FP:decoder.rs-1526 */             }
/* FP:decoder.rs-1527 */             _ => bug!(),
/* FP:decoder.rs-1528 */         }
/* FP:decoder.rs-1529 */     }
/* FP:decoder.rs-1530 */ 
/* FP:decoder.rs-1531 */     #[inline]
/* FP:decoder.rs-1532 */     fn def_key(self, index: DefIndex) -> DefKey {
/* FP:decoder.rs-1533 */         *self
/* FP:decoder.rs-1534 */             .def_key_cache
/* FP:decoder.rs-1535 */             .lock()
/* FP:decoder.rs-1536 */             .entry(index)
/* FP:decoder.rs-1537 */             .or_insert_with(|| self.root.tables.def_keys.get(self, index).unwrap().decode(self))
/* FP:decoder.rs-1538 */     }
/* FP:decoder.rs-1539 */ 
/* FP:decoder.rs-1540 */     // Returns the path leading to the thing with this `id`.
/* FP:decoder.rs-1541 */     fn def_path(self, id: DefIndex) -> DefPath {
/* FP:decoder.rs-1542 */         debug!("def_path(cnum={:?}, id={:?})", self.cnum, id);
/* FP:decoder.rs-1543 */         DefPath::make(self.cnum, id, |parent| self.def_key(parent))
/* FP:decoder.rs-1544 */     }
/* FP:decoder.rs-1545 */ 
/* FP:decoder.rs-1546 */     #[inline]
/* FP:decoder.rs-1547 */     fn def_path_hash(self, index: DefIndex) -> DefPathHash {
/* FP:decoder.rs-1548 */         // This is a hack to workaround the fact that we can't easily encode/decode a Hash64
/* FP:decoder.rs-1549 */         // into the FixedSizeEncoding, as Hash64 lacks a Default impl. A future refactor to
/* FP:decoder.rs-1550 */         // relax the Default restriction will likely fix this.
/* FP:decoder.rs-1551 */         let fingerprint = Fingerprint::new(
/* FP:decoder.rs-1552 */             self.root.stable_crate_id.as_u64(),
/* FP:decoder.rs-1553 */             self.root.tables.def_path_hashes.get(self, index),
/* FP:decoder.rs-1554 */         );
/* FP:decoder.rs-1555 */         DefPathHash::new(self.root.stable_crate_id, fingerprint.split().1)
/* FP:decoder.rs-1556 */     }
/* FP:decoder.rs-1557 */ 
/* FP:decoder.rs-1558 */     #[inline]
/* FP:decoder.rs-1559 */     fn def_path_hash_to_def_index(self, hash: DefPathHash) -> DefIndex {
/* FP:decoder.rs-1560 */         self.def_path_hash_map.def_path_hash_to_def_index(&hash)
/* FP:decoder.rs-1561 */     }
/* FP:decoder.rs-1562 */ 
/* FP:decoder.rs-1563 */     fn expn_hash_to_expn_id(self, sess: &Session, index_guess: u32, hash: ExpnHash) -> ExpnId {
/* FP:decoder.rs-1564 */         debug_assert_eq!(ExpnId::from_hash(hash), None);
/* FP:decoder.rs-1565 */         let index_guess = ExpnIndex::from_u32(index_guess);
/* FP:decoder.rs-1566 */         let old_hash = self.root.expn_hashes.get(self, index_guess).map(|lazy| lazy.decode(self));
/* FP:decoder.rs-1567 */ 
/* FP:decoder.rs-1568 */         let index = if old_hash == Some(hash) {
/* FP:decoder.rs-1569 */             // Fast path: the expn and its index is unchanged from the
/* FP:decoder.rs-1570 */             // previous compilation session. There is no need to decode anything
/* FP:decoder.rs-1571 */             // else.
/* FP:decoder.rs-1572 */             index_guess
/* FP:decoder.rs-1573 */         } else {
/* FP:decoder.rs-1574 */             // Slow path: We need to find out the new `DefIndex` of the provided
/* FP:decoder.rs-1575 */             // `DefPathHash`, if its still exists. This requires decoding every `DefPathHash`
/* FP:decoder.rs-1576 */             // stored in this crate.
/* FP:decoder.rs-1577 */             let map = self.cdata.expn_hash_map.get_or_init(|| {
/* FP:decoder.rs-1578 */                 let end_id = self.root.expn_hashes.size() as u32;
/* FP:decoder.rs-1579 */                 let mut map =
/* FP:decoder.rs-1580 */                     UnhashMap::with_capacity_and_hasher(end_id as usize, Default::default());
/* FP:decoder.rs-1581 */                 for i in 0..end_id {
/* FP:decoder.rs-1582 */                     let i = ExpnIndex::from_u32(i);
/* FP:decoder.rs-1583 */                     if let Some(hash) = self.root.expn_hashes.get(self, i) {
/* FP:decoder.rs-1584 */                         map.insert(hash.decode(self), i);
/* FP:decoder.rs-1585 */                     }
/* FP:decoder.rs-1586 */                 }
/* FP:decoder.rs-1587 */                 map
/* FP:decoder.rs-1588 */             });
/* FP:decoder.rs-1589 */             map[&hash]
/* FP:decoder.rs-1590 */         };
/* FP:decoder.rs-1591 */ 
/* FP:decoder.rs-1592 */         let data = self.root.expn_data.get(self, index).unwrap().decode((self, sess));
/* FP:decoder.rs-1593 */         crate::rustc_span::hygiene::register_expn_id(self.cnum, index, data, hash)
/* FP:decoder.rs-1594 */     }
/* FP:decoder.rs-1595 */ 
/* FP:decoder.rs-1596 */     /// Imports the source_map from an external crate into the source_map of the crate
/* FP:decoder.rs-1597 */     /// currently being compiled (the "local crate").
/* FP:decoder.rs-1598 */     ///
/* FP:decoder.rs-1599 */     /// The import algorithm works analogous to how AST items are inlined from an
/* FP:decoder.rs-1600 */     /// external crate's metadata:
/* FP:decoder.rs-1601 */     /// For every SourceFile in the external source_map an 'inline' copy is created in the
/* FP:decoder.rs-1602 */     /// local source_map. The correspondence relation between external and local
/* FP:decoder.rs-1603 */     /// SourceFiles is recorded in the `ImportedSourceFile` objects returned from this
/* FP:decoder.rs-1604 */     /// function. When an item from an external crate is later inlined into this
/* FP:decoder.rs-1605 */     /// crate, this correspondence information is used to translate the span
/* FP:decoder.rs-1606 */     /// information of the inlined item so that it refers the correct positions in
/* FP:decoder.rs-1607 */     /// the local source_map (see `<decoder::DecodeContext as SpecializedDecoder<Span>>`).
/* FP:decoder.rs-1608 */     ///
/* FP:decoder.rs-1609 */     /// The import algorithm in the function below will reuse SourceFiles already
/* FP:decoder.rs-1610 */     /// existing in the local source_map. For example, even if the SourceFile of some
/* FP:decoder.rs-1611 */     /// source file of libstd gets imported many times, there will only ever be
/* FP:decoder.rs-1612 */     /// one SourceFile object for the corresponding file in the local source_map.
/* FP:decoder.rs-1613 */     ///
/* FP:decoder.rs-1614 */     /// Note that imported SourceFiles do not actually contain the source code of the
/* FP:decoder.rs-1615 */     /// file they represent, just information about length, line breaks, and
/* FP:decoder.rs-1616 */     /// multibyte characters. This information is enough to generate valid debuginfo
/* FP:decoder.rs-1617 */     /// for items inlined from other crates.
/* FP:decoder.rs-1618 */     ///
/* FP:decoder.rs-1619 */     /// Proc macro crates don't currently export spans, so this function does not have
/* FP:decoder.rs-1620 */     /// to work for them.
/* FP:decoder.rs-1621 */     fn imported_source_file(self, source_file_index: u32, sess: &Session) -> ImportedSourceFile {
/* FP:decoder.rs-1622 */         fn filter<'a>(
/* FP:decoder.rs-1623 */             sess: &Session,
/* FP:decoder.rs-1624 */             real_source_base_dir: &Option<PathBuf>,
/* FP:decoder.rs-1625 */             path: Option<&'a Path>,
/* FP:decoder.rs-1626 */         ) -> Option<&'a Path> {
/* FP:decoder.rs-1627 */             path.filter(|_| {
/* FP:decoder.rs-1628 */                 // Only spend time on further checks if we have what to translate *to*.
/* FP:decoder.rs-1629 */                 real_source_base_dir.is_some()
/* FP:decoder.rs-1630 */                 // Some tests need the translation to be always skipped.
/* FP:decoder.rs-1631 */                 && sess.opts.unstable_opts.translate_remapped_path_to_local_path
/* FP:decoder.rs-1632 */             })
/* FP:decoder.rs-1633 */             .filter(|virtual_dir| {
/* FP:decoder.rs-1634 */                 // Don't translate away `/rustc/$hash` if we're still remapping to it,
/* FP:decoder.rs-1635 */                 // since that means we're still building `std`/`rustc` that need it,
/* FP:decoder.rs-1636 */                 // and we don't want the real path to leak into codegen/debuginfo.
/* FP:decoder.rs-1637 */                 !sess.opts.remap_path_prefix.iter().any(|(_from, to)| to == virtual_dir)
/* FP:decoder.rs-1638 */             })
/* FP:decoder.rs-1639 */         }
/* FP:decoder.rs-1640 */ 
/* FP:decoder.rs-1641 */         let try_to_translate_virtual_to_real =
/* FP:decoder.rs-1642 */             |virtual_source_base_dir: Option<&str>,
/* FP:decoder.rs-1643 */              real_source_base_dir: &Option<PathBuf>,
/* FP:decoder.rs-1644 */              name: &mut crate::rustc_span::FileName| {
/* FP:decoder.rs-1645 */                 let virtual_source_base_dir = [
/* FP:decoder.rs-1646 */                     filter(sess, real_source_base_dir, virtual_source_base_dir.map(Path::new)),
/* FP:decoder.rs-1647 */                     filter(
/* FP:decoder.rs-1648 */                         sess,
/* FP:decoder.rs-1649 */                         real_source_base_dir,
/* FP:decoder.rs-1650 */                         sess.opts.unstable_opts.simulate_remapped_rust_src_base.as_deref(),
/* FP:decoder.rs-1651 */                     ),
/* FP:decoder.rs-1652 */                 ];
/* FP:decoder.rs-1653 */ 
/* FP:decoder.rs-1654 */                 debug!(
/* FP:decoder.rs-1655 */                     "try_to_translate_virtual_to_real(name={:?}): \
/* FP:decoder.rs-1656 */                      virtual_source_base_dir={:?}, real_source_base_dir={:?}",
/* FP:decoder.rs-1657 */                     name, virtual_source_base_dir, real_source_base_dir,
/* FP:decoder.rs-1658 */                 );
/* FP:decoder.rs-1659 */ 
/* FP:decoder.rs-1660 */                 for virtual_dir in virtual_source_base_dir.iter().flatten() {
/* FP:decoder.rs-1661 */                     if let Some(real_dir) = &real_source_base_dir
/* FP:decoder.rs-1662 */                         && let crate::rustc_span::FileName::Real(old_name) = name
/* FP:decoder.rs-1663 */                         && let crate::rustc_span::RealFileName::Remapped { local_path: _, virtual_name } =
/* FP:decoder.rs-1664 */                             old_name
/* FP:decoder.rs-1665 */                         && let Ok(rest) = virtual_name.strip_prefix(virtual_dir)
/* FP:decoder.rs-1666 */                     {
/* FP:decoder.rs-1667 */                         let new_path = real_dir.join(rest);
/* FP:decoder.rs-1668 */ 
/* FP:decoder.rs-1669 */                         debug!(
/* FP:decoder.rs-1670 */                             "try_to_translate_virtual_to_real: `{}` -> `{}`",
/* FP:decoder.rs-1671 */                             virtual_name.display(),
/* FP:decoder.rs-1672 */                             new_path.display(),
/* FP:decoder.rs-1673 */                         );
/* FP:decoder.rs-1674 */ 
/* FP:decoder.rs-1675 */                         // Check if the translated real path is affected by any user-requested
/* FP:decoder.rs-1676 */                         // remaps via --remap-path-prefix. Apply them if so.
/* FP:decoder.rs-1677 */                         // Note that this is a special case for imported rust-src paths specified by
/* FP:decoder.rs-1678 */                         // https://rust-lang.github.io/rfcs/3127-trim-paths.html#handling-sysroot-paths.
/* FP:decoder.rs-1679 */                         // Other imported paths are not currently remapped (see #66251).
/* FP:decoder.rs-1680 */                         let (user_remapped, applied) =
/* FP:decoder.rs-1681 */                             sess.source_map().path_mapping().map_prefix(&new_path);
/* FP:decoder.rs-1682 */                         let new_name = if applied {
/* FP:decoder.rs-1683 */                             crate::rustc_span::RealFileName::Remapped {
/* FP:decoder.rs-1684 */                                 local_path: Some(new_path.clone()),
/* FP:decoder.rs-1685 */                                 virtual_name: user_remapped.to_path_buf(),
/* FP:decoder.rs-1686 */                             }
/* FP:decoder.rs-1687 */                         } else {
/* FP:decoder.rs-1688 */                             crate::rustc_span::RealFileName::LocalPath(new_path)
/* FP:decoder.rs-1689 */                         };
/* FP:decoder.rs-1690 */                         *old_name = new_name;
/* FP:decoder.rs-1691 */                     }
/* FP:decoder.rs-1692 */                 }
/* FP:decoder.rs-1693 */             };
/* FP:decoder.rs-1694 */ 
/* FP:decoder.rs-1695 */         let try_to_translate_real_to_virtual =
/* FP:decoder.rs-1696 */             |virtual_source_base_dir: Option<&str>,
/* FP:decoder.rs-1697 */              real_source_base_dir: &Option<PathBuf>,
/* FP:decoder.rs-1698 */              subdir: &str,
/* FP:decoder.rs-1699 */              name: &mut crate::rustc_span::FileName| {
/* FP:decoder.rs-1700 */                 if let Some(virtual_dir) = &sess.opts.unstable_opts.simulate_remapped_rust_src_base
/* FP:decoder.rs-1701 */                     && let Some(real_dir) = real_source_base_dir
/* FP:decoder.rs-1702 */                     && let crate::rustc_span::FileName::Real(old_name) = name
/* FP:decoder.rs-1703 */                 {
/* FP:decoder.rs-1704 */                     let relative_path = match old_name {
/* FP:decoder.rs-1705 */                         crate::rustc_span::RealFileName::LocalPath(local) => {
/* FP:decoder.rs-1706 */                             local.strip_prefix(real_dir).ok()
/* FP:decoder.rs-1707 */                         }
/* FP:decoder.rs-1708 */                         crate::rustc_span::RealFileName::Remapped { virtual_name, .. } => {
/* FP:decoder.rs-1709 */                             virtual_source_base_dir
/* FP:decoder.rs-1710 */                                 .and_then(|virtual_dir| virtual_name.strip_prefix(virtual_dir).ok())
/* FP:decoder.rs-1711 */                         }
/* FP:decoder.rs-1712 */                     };
/* FP:decoder.rs-1713 */                     debug!(
/* FP:decoder.rs-1714 */                         ?relative_path,
/* FP:decoder.rs-1715 */                         ?virtual_dir,
/* FP:decoder.rs-1716 */                         ?subdir,
/* FP:decoder.rs-1717 */                         "simulate_remapped_rust_src_base"
/* FP:decoder.rs-1718 */                     );
/* FP:decoder.rs-1719 */                     if let Some(rest) = relative_path.and_then(|p| p.strip_prefix(subdir).ok()) {
/* FP:decoder.rs-1720 */                         *old_name = crate::rustc_span::RealFileName::Remapped {
/* FP:decoder.rs-1721 */                             local_path: None,
/* FP:decoder.rs-1722 */                             virtual_name: virtual_dir.join(subdir).join(rest),
/* FP:decoder.rs-1723 */                         };
/* FP:decoder.rs-1724 */                     }
/* FP:decoder.rs-1725 */                 }
/* FP:decoder.rs-1726 */             };
/* FP:decoder.rs-1727 */ 
/* FP:decoder.rs-1728 */         let mut import_info = self.cdata.source_map_import_info.lock();
/* FP:decoder.rs-1729 */         for _ in import_info.len()..=(source_file_index as usize) {
/* FP:decoder.rs-1730 */             import_info.push(None);
/* FP:decoder.rs-1731 */         }
/* FP:decoder.rs-1732 */         import_info[source_file_index as usize]
/* FP:decoder.rs-1733 */             .get_or_insert_with(|| {
/* FP:decoder.rs-1734 */                 let source_file_to_import = self
/* FP:decoder.rs-1735 */                     .root
/* FP:decoder.rs-1736 */                     .source_map
/* FP:decoder.rs-1737 */                     .get(self, source_file_index)
/* FP:decoder.rs-1738 */                     .expect("missing source file")
/* FP:decoder.rs-1739 */                     .decode(self);
/* FP:decoder.rs-1740 */ 
/* FP:decoder.rs-1741 */                 // We can't reuse an existing SourceFile, so allocate a new one
/* FP:decoder.rs-1742 */                 // containing the information we need.
/* FP:decoder.rs-1743 */                 let original_end_pos = source_file_to_import.end_position();
/* FP:decoder.rs-1744 */                 let crate::rustc_span::SourceFile {
/* FP:decoder.rs-1745 */                     mut name,
/* FP:decoder.rs-1746 */                     src_hash,
/* FP:decoder.rs-1747 */                     checksum_hash,
/* FP:decoder.rs-1748 */                     start_pos: original_start_pos,
/* FP:decoder.rs-1749 */                     source_len,
/* FP:decoder.rs-1750 */                     lines,
/* FP:decoder.rs-1751 */                     multibyte_chars,
/* FP:decoder.rs-1752 */                     normalized_pos,
/* FP:decoder.rs-1753 */                     stable_id,
/* FP:decoder.rs-1754 */                     ..
/* FP:decoder.rs-1755 */                 } = source_file_to_import;
/* FP:decoder.rs-1756 */ 
/* FP:decoder.rs-1757 */                 // If this file is under $sysroot/lib/rustlib/src/
/* FP:decoder.rs-1758 */                 // and the user wish to simulate remapping with -Z simulate-remapped-rust-src-base,
/* FP:decoder.rs-1759 */                 // then we change `name` to a similar state as if the rust was bootstrapped
/* FP:decoder.rs-1760 */                 // with `remap-debuginfo = true`.
/* FP:decoder.rs-1761 */                 // This is useful for testing so that tests about the effects of
/* FP:decoder.rs-1762 */                 // `try_to_translate_virtual_to_real` don't have to worry about how the
/* FP:decoder.rs-1763 */                 // compiler is bootstrapped.
/* FP:decoder.rs-1764 */                 try_to_translate_real_to_virtual(
/* FP:decoder.rs-1765 */                     option_env!("CFG_VIRTUAL_RUST_SOURCE_BASE_DIR"),
/* FP:decoder.rs-1766 */                     &sess.opts.real_rust_source_base_dir,
/* FP:decoder.rs-1767 */                     "library",
/* FP:decoder.rs-1768 */                     &mut name,
/* FP:decoder.rs-1769 */                 );
/* FP:decoder.rs-1770 */ 
/* FP:decoder.rs-1771 */                 // If this file is under $sysroot/lib/rustlib/rustc-src/
/* FP:decoder.rs-1772 */                 // and the user wish to simulate remapping with -Z simulate-remapped-rust-src-base,
/* FP:decoder.rs-1773 */                 // then we change `name` to a similar state as if the rust was bootstrapped
/* FP:decoder.rs-1774 */                 // with `remap-debuginfo = true`.
/* FP:decoder.rs-1775 */                 try_to_translate_real_to_virtual(
/* FP:decoder.rs-1776 */                     option_env!("CFG_VIRTUAL_RUSTC_DEV_SOURCE_BASE_DIR"),
/* FP:decoder.rs-1777 */                     &sess.opts.real_rustc_dev_source_base_dir,
/* FP:decoder.rs-1778 */                     "compiler",
/* FP:decoder.rs-1779 */                     &mut name,
/* FP:decoder.rs-1780 */                 );
/* FP:decoder.rs-1781 */ 
/* FP:decoder.rs-1782 */                 // If this file's path has been remapped to `/rustc/$hash`,
/* FP:decoder.rs-1783 */                 // we might be able to reverse that.
/* FP:decoder.rs-1784 */                 //
/* FP:decoder.rs-1785 */                 // NOTE: if you update this, you might need to also update bootstrap's code for generating
/* FP:decoder.rs-1786 */                 // the `rust-src` component in `Src::run` in `src/bootstrap/dist.rs`.
/* FP:decoder.rs-1787 */                 try_to_translate_virtual_to_real(
/* FP:decoder.rs-1788 */                     option_env!("CFG_VIRTUAL_RUST_SOURCE_BASE_DIR"),
/* FP:decoder.rs-1789 */                     &sess.opts.real_rust_source_base_dir,
/* FP:decoder.rs-1790 */                     &mut name,
/* FP:decoder.rs-1791 */                 );
/* FP:decoder.rs-1792 */ 
/* FP:decoder.rs-1793 */                 // If this file's path has been remapped to `/rustc-dev/$hash`,
/* FP:decoder.rs-1794 */                 // we might be able to reverse that.
/* FP:decoder.rs-1795 */                 //
/* FP:decoder.rs-1796 */                 // NOTE: if you update this, you might need to also update bootstrap's code for generating
/* FP:decoder.rs-1797 */                 // the `rustc-dev` component in `Src::run` in `src/bootstrap/dist.rs`.
/* FP:decoder.rs-1798 */                 try_to_translate_virtual_to_real(
/* FP:decoder.rs-1799 */                     option_env!("CFG_VIRTUAL_RUSTC_DEV_SOURCE_BASE_DIR"),
/* FP:decoder.rs-1800 */                     &sess.opts.real_rustc_dev_source_base_dir,
/* FP:decoder.rs-1801 */                     &mut name,
/* FP:decoder.rs-1802 */                 );
/* FP:decoder.rs-1803 */ 
/* FP:decoder.rs-1804 */                 let local_version = sess.source_map().new_imported_source_file(
/* FP:decoder.rs-1805 */                     name,
/* FP:decoder.rs-1806 */                     src_hash,
/* FP:decoder.rs-1807 */                     checksum_hash,
/* FP:decoder.rs-1808 */                     stable_id,
/* FP:decoder.rs-1809 */                     source_len.to_u32(),
/* FP:decoder.rs-1810 */                     self.cnum,
/* FP:decoder.rs-1811 */                     lines,
/* FP:decoder.rs-1812 */                     multibyte_chars,
/* FP:decoder.rs-1813 */                     normalized_pos,
/* FP:decoder.rs-1814 */                     source_file_index,
/* FP:decoder.rs-1815 */                 );
/* FP:decoder.rs-1816 */                 debug!(
/* FP:decoder.rs-1817 */                     "CrateMetaData::imported_source_files alloc \
/* FP:decoder.rs-1818 */                          source_file {:?} original (start_pos {:?} source_len {:?}) \
/* FP:decoder.rs-1819 */                          translated (start_pos {:?} source_len {:?})",
/* FP:decoder.rs-1820 */                     local_version.name,
/* FP:decoder.rs-1821 */                     original_start_pos,
/* FP:decoder.rs-1822 */                     source_len,
/* FP:decoder.rs-1823 */                     local_version.start_pos,
/* FP:decoder.rs-1824 */                     local_version.source_len
/* FP:decoder.rs-1825 */                 );
/* FP:decoder.rs-1826 */ 
/* FP:decoder.rs-1827 */                 ImportedSourceFile {
/* FP:decoder.rs-1828 */                     original_start_pos,
/* FP:decoder.rs-1829 */                     original_end_pos,
/* FP:decoder.rs-1830 */                     translated_source_file: local_version,
/* FP:decoder.rs-1831 */                 }
/* FP:decoder.rs-1832 */             })
/* FP:decoder.rs-1833 */             .clone()
/* FP:decoder.rs-1834 */     }
/* FP:decoder.rs-1835 */ 
/* FP:decoder.rs-1836 */     fn get_attr_flags(self, index: DefIndex) -> AttrFlags {
/* FP:decoder.rs-1837 */         self.root.tables.attr_flags.get(self, index)
/* FP:decoder.rs-1838 */     }
/* FP:decoder.rs-1839 */ 
/* FP:decoder.rs-1840 */     fn get_intrinsic(self, index: DefIndex) -> Option<ty::IntrinsicDef> {
/* FP:decoder.rs-1841 */         self.root.tables.intrinsic.get(self, index).map(|d| d.decode(self))
/* FP:decoder.rs-1842 */     }
/* FP:decoder.rs-1843 */ 
/* FP:decoder.rs-1844 */     fn get_doc_link_resolutions(self, index: DefIndex) -> DocLinkResMap {
/* FP:decoder.rs-1845 */         self.root
/* FP:decoder.rs-1846 */             .tables
/* FP:decoder.rs-1847 */             .doc_link_resolutions
/* FP:decoder.rs-1848 */             .get(self, index)
/* FP:decoder.rs-1849 */             .expect("no resolutions for a doc link")
/* FP:decoder.rs-1850 */             .decode(self)
/* FP:decoder.rs-1851 */     }
/* FP:decoder.rs-1852 */ 
/* FP:decoder.rs-1853 */     fn get_doc_link_traits_in_scope(self, index: DefIndex) -> impl Iterator<Item = DefId> {
/* FP:decoder.rs-1854 */         self.root
/* FP:decoder.rs-1855 */             .tables
/* FP:decoder.rs-1856 */             .doc_link_traits_in_scope
/* FP:decoder.rs-1857 */             .get(self, index)
/* FP:decoder.rs-1858 */             .expect("no traits in scope for a doc link")
/* FP:decoder.rs-1859 */             .decode(self)
/* FP:decoder.rs-1860 */     }
/* FP:decoder.rs-1861 */ }
/* FP:decoder.rs-1862 */ 
/* FP:decoder.rs-1863 */ impl CrateMetadata {
/* FP:decoder.rs-1864 */     pub(crate) fn new(
/* FP:decoder.rs-1865 */         sess: &Session,
/* FP:decoder.rs-1866 */         cstore: &CStore,
/* FP:decoder.rs-1867 */         blob: MetadataBlob,
/* FP:decoder.rs-1868 */         root: CrateRoot,
/* FP:decoder.rs-1869 */         raw_proc_macros: Option<&'static [ProcMacro]>,
/* FP:decoder.rs-1870 */         cnum: CrateNum,
/* FP:decoder.rs-1871 */         cnum_map: CrateNumMap,
/* FP:decoder.rs-1872 */         dep_kind: CrateDepKind,
/* FP:decoder.rs-1873 */         source: CrateSource,
/* FP:decoder.rs-1874 */         private_dep: bool,
/* FP:decoder.rs-1875 */         host_hash: Option<Svh>,
/* FP:decoder.rs-1876 */     ) -> CrateMetadata {
/* FP:decoder.rs-1877 */         let trait_impls = root
/* FP:decoder.rs-1878 */             .impls
/* FP:decoder.rs-1879 */             .decode((&blob, sess))
/* FP:decoder.rs-1880 */             .map(|trait_impls| (trait_impls.trait_id, trait_impls.impls))
/* FP:decoder.rs-1881 */             .collect();
/* FP:decoder.rs-1882 */         let alloc_decoding_state =
/* FP:decoder.rs-1883 */             AllocDecodingState::new(root.interpret_alloc_index.decode(&blob).collect());
/* FP:decoder.rs-1884 */         let dependencies = cnum_map.iter().copied().collect();
/* FP:decoder.rs-1885 */ 
/* FP:decoder.rs-1886 */         // Pre-decode the DefPathHash->DefIndex table. This is a cheap operation
/* FP:decoder.rs-1887 */         // that does not copy any data. It just does some data verification.
/* FP:decoder.rs-1888 */         let def_path_hash_map = root.def_path_hash_map.decode(&blob);
/* FP:decoder.rs-1889 */ 
/* FP:decoder.rs-1890 */         let mut cdata = CrateMetadata {
/* FP:decoder.rs-1891 */             blob,
/* FP:decoder.rs-1892 */             root,
/* FP:decoder.rs-1893 */             trait_impls,
/* FP:decoder.rs-1894 */             incoherent_impls: Default::default(),
/* FP:decoder.rs-1895 */             raw_proc_macros,
/* FP:decoder.rs-1896 */             source_map_import_info: Lock::new(Vec::new()),
/* FP:decoder.rs-1897 */             def_path_hash_map,
/* FP:decoder.rs-1898 */             expn_hash_map: Default::default(),
/* FP:decoder.rs-1899 */             alloc_decoding_state,
/* FP:decoder.rs-1900 */             cnum,
/* FP:decoder.rs-1901 */             cnum_map,
/* FP:decoder.rs-1902 */             dependencies,
/* FP:decoder.rs-1903 */             dep_kind,
/* FP:decoder.rs-1904 */             source: Arc::new(source),
/* FP:decoder.rs-1905 */             private_dep,
/* FP:decoder.rs-1906 */             host_hash,
/* FP:decoder.rs-1907 */             used: false,
/* FP:decoder.rs-1908 */             extern_crate: None,
/* FP:decoder.rs-1909 */             hygiene_context: Default::default(),
/* FP:decoder.rs-1910 */             def_key_cache: Default::default(),
/* FP:decoder.rs-1911 */         };
/* FP:decoder.rs-1912 */ 
/* FP:decoder.rs-1913 */         // Need `CrateMetadataRef` to decode `DefId`s in simplified types.
/* FP:decoder.rs-1914 */         cdata.incoherent_impls = cdata
/* FP:decoder.rs-1915 */             .root
/* FP:decoder.rs-1916 */             .incoherent_impls
/* FP:decoder.rs-1917 */             .decode(CrateMetadataRef { cdata: &cdata, cstore })
/* FP:decoder.rs-1918 */             .map(|incoherent_impls| (incoherent_impls.self_ty, incoherent_impls.impls))
/* FP:decoder.rs-1919 */             .collect();
/* FP:decoder.rs-1920 */ 
/* FP:decoder.rs-1921 */         cdata
/* FP:decoder.rs-1922 */     }
/* FP:decoder.rs-1923 */ 
/* FP:decoder.rs-1924 */     pub(crate) fn dependencies(&self) -> impl Iterator<Item = CrateNum> {
/* FP:decoder.rs-1925 */         self.dependencies.iter().copied()
/* FP:decoder.rs-1926 */     }
/* FP:decoder.rs-1927 */ 
/* FP:decoder.rs-1928 */     pub(crate) fn target_modifiers(&self) -> TargetModifiers {
/* FP:decoder.rs-1929 */         self.root.decode_target_modifiers(&self.blob).collect()
/* FP:decoder.rs-1930 */     }
/* FP:decoder.rs-1931 */ 
/* FP:decoder.rs-1932 */     /// Keep `new_extern_crate` if it looks better in diagnostics
/* FP:decoder.rs-1933 */     pub(crate) fn update_extern_crate_diagnostics(
/* FP:decoder.rs-1934 */         &mut self,
/* FP:decoder.rs-1935 */         new_extern_crate: ExternCrate,
/* FP:decoder.rs-1936 */     ) -> bool {
/* FP:decoder.rs-1937 */         let update =
/* FP:decoder.rs-1938 */             self.extern_crate.as_ref().is_none_or(|old| old.rank() < new_extern_crate.rank());
/* FP:decoder.rs-1939 */         if update {
/* FP:decoder.rs-1940 */             self.extern_crate = Some(new_extern_crate);
/* FP:decoder.rs-1941 */         }
/* FP:decoder.rs-1942 */         update
/* FP:decoder.rs-1943 */     }
/* FP:decoder.rs-1944 */ 
/* FP:decoder.rs-1945 */     pub(crate) fn source(&self) -> &CrateSource {
/* FP:decoder.rs-1946 */         &*self.source
/* FP:decoder.rs-1947 */     }
/* FP:decoder.rs-1948 */ 
/* FP:decoder.rs-1949 */     pub(crate) fn dep_kind(&self) -> CrateDepKind {
/* FP:decoder.rs-1950 */         self.dep_kind
/* FP:decoder.rs-1951 */     }
/* FP:decoder.rs-1952 */ 
/* FP:decoder.rs-1953 */     pub(crate) fn set_dep_kind(&mut self, dep_kind: CrateDepKind) {
/* FP:decoder.rs-1954 */         self.dep_kind = dep_kind;
/* FP:decoder.rs-1955 */     }
/* FP:decoder.rs-1956 */ 
/* FP:decoder.rs-1957 */     pub(crate) fn update_and_private_dep(&mut self, private_dep: bool) {
/* FP:decoder.rs-1958 */         self.private_dep &= private_dep;
/* FP:decoder.rs-1959 */     }
/* FP:decoder.rs-1960 */ 
/* FP:decoder.rs-1961 */     pub(crate) fn used(&self) -> bool {
/* FP:decoder.rs-1962 */         self.used
/* FP:decoder.rs-1963 */     }
/* FP:decoder.rs-1964 */ 
/* FP:decoder.rs-1965 */     pub(crate) fn required_panic_strategy(&self) -> Option<PanicStrategy> {
/* FP:decoder.rs-1966 */         self.root.required_panic_strategy
/* FP:decoder.rs-1967 */     }
/* FP:decoder.rs-1968 */ 
/* FP:decoder.rs-1969 */     pub(crate) fn needs_panic_runtime(&self) -> bool {
/* FP:decoder.rs-1970 */         self.root.needs_panic_runtime
/* FP:decoder.rs-1971 */     }
/* FP:decoder.rs-1972 */ 
/* FP:decoder.rs-1973 */     pub(crate) fn is_private_dep(&self) -> bool {
/* FP:decoder.rs-1974 */         self.private_dep
/* FP:decoder.rs-1975 */     }
/* FP:decoder.rs-1976 */ 
/* FP:decoder.rs-1977 */     pub(crate) fn is_panic_runtime(&self) -> bool {
/* FP:decoder.rs-1978 */         self.root.panic_runtime
/* FP:decoder.rs-1979 */     }
/* FP:decoder.rs-1980 */ 
/* FP:decoder.rs-1981 */     pub(crate) fn is_profiler_runtime(&self) -> bool {
/* FP:decoder.rs-1982 */         self.root.profiler_runtime
/* FP:decoder.rs-1983 */     }
/* FP:decoder.rs-1984 */ 
/* FP:decoder.rs-1985 */     pub(crate) fn is_compiler_builtins(&self) -> bool {
/* FP:decoder.rs-1986 */         self.root.compiler_builtins
/* FP:decoder.rs-1987 */     }
/* FP:decoder.rs-1988 */ 
/* FP:decoder.rs-1989 */     pub(crate) fn needs_allocator(&self) -> bool {
/* FP:decoder.rs-1990 */         self.root.needs_allocator
/* FP:decoder.rs-1991 */     }
/* FP:decoder.rs-1992 */ 
/* FP:decoder.rs-1993 */     pub(crate) fn has_global_allocator(&self) -> bool {
/* FP:decoder.rs-1994 */         self.root.has_global_allocator
/* FP:decoder.rs-1995 */     }
/* FP:decoder.rs-1996 */ 
/* FP:decoder.rs-1997 */     pub(crate) fn has_alloc_error_handler(&self) -> bool {
/* FP:decoder.rs-1998 */         self.root.has_alloc_error_handler
/* FP:decoder.rs-1999 */     }
/* FP:decoder.rs-2000 */ 
/* FP:decoder.rs-2001 */     pub(crate) fn has_default_lib_allocator(&self) -> bool {
/* FP:decoder.rs-2002 */         self.root.has_default_lib_allocator
/* FP:decoder.rs-2003 */     }
/* FP:decoder.rs-2004 */ 
/* FP:decoder.rs-2005 */     pub(crate) fn is_proc_macro_crate(&self) -> bool {
/* FP:decoder.rs-2006 */         self.root.is_proc_macro_crate()
/* FP:decoder.rs-2007 */     }
/* FP:decoder.rs-2008 */ 
/* FP:decoder.rs-2009 */     pub(crate) fn proc_macros_for_crate(
/* FP:decoder.rs-2010 */         &self,
/* FP:decoder.rs-2011 */         krate: CrateNum,
/* FP:decoder.rs-2012 */         cstore: &CStore,
/* FP:decoder.rs-2013 */     ) -> impl Iterator<Item = DefId> {
/* FP:decoder.rs-2014 */         gen move {
/* FP:decoder.rs-2015 */             for def_id in self.root.proc_macro_data.as_ref().into_iter().flat_map(move |data| {
/* FP:decoder.rs-2016 */                 data.macros
/* FP:decoder.rs-2017 */                     .decode(CrateMetadataRef { cdata: self, cstore })
/* FP:decoder.rs-2018 */                     .map(move |index| DefId { index, krate })
/* FP:decoder.rs-2019 */             }) {
/* FP:decoder.rs-2020 */                 yield def_id;
/* FP:decoder.rs-2021 */             }
/* FP:decoder.rs-2022 */         }
/* FP:decoder.rs-2023 */     }
/* FP:decoder.rs-2024 */ 
/* FP:decoder.rs-2025 */     pub(crate) fn name(&self) -> Symbol {
/* FP:decoder.rs-2026 */         self.root.header.name
/* FP:decoder.rs-2027 */     }
/* FP:decoder.rs-2028 */ 
/* FP:decoder.rs-2029 */     pub(crate) fn hash(&self) -> Svh {
/* FP:decoder.rs-2030 */         self.root.header.hash
/* FP:decoder.rs-2031 */     }
/* FP:decoder.rs-2032 */ 
/* FP:decoder.rs-2033 */     pub(crate) fn has_async_drops(&self) -> bool {
/* FP:decoder.rs-2034 */         self.root.tables.adt_async_destructor.len > 0
/* FP:decoder.rs-2035 */     }
/* FP:decoder.rs-2036 */ 
/* FP:decoder.rs-2037 */     fn num_def_ids(&self) -> usize {
/* FP:decoder.rs-2038 */         self.root.tables.def_keys.size()
/* FP:decoder.rs-2039 */     }
/* FP:decoder.rs-2040 */ 
/* FP:decoder.rs-2041 */     fn local_def_id(&self, index: DefIndex) -> DefId {
/* FP:decoder.rs-2042 */         DefId { krate: self.cnum, index }
/* FP:decoder.rs-2043 */     }
/* FP:decoder.rs-2044 */ 
/* FP:decoder.rs-2045 */     // Translate a DefId from the current compilation environment to a DefId
/* FP:decoder.rs-2046 */     // for an external crate.
/* FP:decoder.rs-2047 */     fn reverse_translate_def_id(&self, did: DefId) -> Option<DefId> {
/* FP:decoder.rs-2048 */         for (local, &global) in self.cnum_map.iter_enumerated() {
/* FP:decoder.rs-2049 */             if global == did.krate {
/* FP:decoder.rs-2050 */                 return Some(DefId { krate: local, index: did.index });
/* FP:decoder.rs-2051 */             }
/* FP:decoder.rs-2052 */         }
/* FP:decoder.rs-2053 */ 
/* FP:decoder.rs-2054 */         None
/* FP:decoder.rs-2055 */     }
/* FP:decoder.rs-2056 */ }