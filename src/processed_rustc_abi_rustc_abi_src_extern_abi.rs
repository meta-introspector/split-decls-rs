/* FP:extern_abi.rs-0001 */ use std::cmp::Ordering;
/* FP:extern_abi.rs-0002 */ use std::fmt;
/* FP:extern_abi.rs-0003 */ use std::hash::{Hash, Hasher};
/* FP:extern_abi.rs-0004 */ 
/* FP:extern_abi.rs-0005 */ #[cfg(feature = "nightly")]
/* FP:extern_abi.rs-0006 */ use crate::rustc_data_structures::stable_hasher::{HashStable, StableHasher, StableOrd};
/* FP:extern_abi.rs-0007 */ #[cfg(feature = "nightly")]
/* FP:extern_abi.rs-0008 */ use rustc_macros::{Decodable, Encodable};
/* FP:extern_abi.rs-0009 */ #[cfg(feature = "nightly")]
/* FP:extern_abi.rs-0010 */ use crate::rustc_complete::Symbol;
/* FP:extern_abi.rs-0011 */ 
/* FP:extern_abi.rs-0012 */ use crate::AbiFromStrErr;
/* FP:extern_abi.rs-0013 */ 
/* FP:extern_abi.rs-0014 */ #[cfg(test)]
/* FP:extern_abi.rs-0016 */ 
/* FP:extern_abi.rs-0017 */ /// ABI we expect to see within `extern "{abi}"`
/* FP:extern_abi.rs-0018 */ #[derive(Clone, Copy, Debug)]
/* FP:extern_abi.rs-0019 */ #[cfg_attr(feature = "nightly", derive(Encodable, Decodable))]
/* FP:extern_abi.rs-0020 */ pub enum ExternAbi {
/* FP:extern_abi.rs-0021 */     /* universal */
/* FP:extern_abi.rs-0022 */     /// presumed C ABI for the platform
/* FP:extern_abi.rs-0023 */     C {
/* FP:extern_abi.rs-0024 */         unwind: bool,
/* FP:extern_abi.rs-0025 */     },
/* FP:extern_abi.rs-0026 */     /// ABI of the "system" interface, e.g. the Win32 API, always "aliasing"
/* FP:extern_abi.rs-0027 */     System {
/* FP:extern_abi.rs-0028 */         unwind: bool,
/* FP:extern_abi.rs-0029 */     },
/* FP:extern_abi.rs-0030 */ 
/* FP:extern_abi.rs-0031 */     /// that's us!
/* FP:extern_abi.rs-0032 */     Rust,
/* FP:extern_abi.rs-0033 */     /// the mostly-unused `unboxed_closures` ABI, effectively now an impl detail unless someone
/* FP:extern_abi.rs-0034 */     /// puts in the work to make it viable again... but would we need a special ABI?
/* FP:extern_abi.rs-0035 */     RustCall,
/* FP:extern_abi.rs-0036 */     /// For things unlikely to be called, where reducing register pressure in
/* FP:extern_abi.rs-0037 */     /// `extern "Rust"` callers is worth paying extra cost in the callee.
/* FP:extern_abi.rs-0038 */     /// Stronger than just `#[cold]` because `fn` pointers might be incompatible.
/* FP:extern_abi.rs-0039 */     RustCold,
/* FP:extern_abi.rs-0040 */ 
/* FP:extern_abi.rs-0041 */     /// An always-invalid ABI that's used to test "this ABI is not supported by this platform"
/* FP:extern_abi.rs-0042 */     /// in a platform-agnostic way.
/* FP:extern_abi.rs-0043 */     RustInvalid,
/* FP:extern_abi.rs-0044 */ 
/* FP:extern_abi.rs-0045 */     /// Unstable impl detail that directly uses Rust types to describe the ABI to LLVM.
/* FP:extern_abi.rs-0046 */     /// Even normally-compatible Rust types can become ABI-incompatible with this ABI!
/* FP:extern_abi.rs-0047 */     Unadjusted,
/* FP:extern_abi.rs-0048 */ 
/* FP:extern_abi.rs-0049 */     /// An ABI that rustc does not know how to call or define. Functions with this ABI can
/* FP:extern_abi.rs-0050 */     /// only be created using `#[naked]` functions or `extern "custom"` blocks, and can only
/* FP:extern_abi.rs-0051 */     /// be called from inline assembly.
/* FP:extern_abi.rs-0052 */     Custom,
/* FP:extern_abi.rs-0053 */ 
/* FP:extern_abi.rs-0054 */     /// UEFI ABI, usually an alias of C, but sometimes an arch-specific alias
/* FP:extern_abi.rs-0055 */     /// and only valid on platforms that have a UEFI standard
/* FP:extern_abi.rs-0056 */     EfiApi,
/* FP:extern_abi.rs-0057 */ 
/* FP:extern_abi.rs-0058 */     /* arm */
/* FP:extern_abi.rs-0059 */     /// Arm Architecture Procedure Call Standard, sometimes `ExternAbi::C` is an alias for this
/* FP:extern_abi.rs-0060 */     Aapcs {
/* FP:extern_abi.rs-0061 */         unwind: bool,
/* FP:extern_abi.rs-0062 */     },
/* FP:extern_abi.rs-0063 */     /// extremely constrained barely-C ABI for TrustZone
/* FP:extern_abi.rs-0064 */     CmseNonSecureCall,
/* FP:extern_abi.rs-0065 */     /// extremely constrained barely-C ABI for TrustZone
/* FP:extern_abi.rs-0066 */     CmseNonSecureEntry,
/* FP:extern_abi.rs-0067 */ 
/* FP:extern_abi.rs-0068 */     /* gpu */
/* FP:extern_abi.rs-0069 */     /// An entry-point function called by the GPU's host
/* FP:extern_abi.rs-0070 */     // FIXME: should not be callable from Rust on GPU targets, is for host's use only
/* FP:extern_abi.rs-0071 */     GpuKernel,
/* FP:extern_abi.rs-0072 */     /// An entry-point function called by the GPU's host
/* FP:extern_abi.rs-0073 */     // FIXME: why do we have two of these?
/* FP:extern_abi.rs-0074 */     PtxKernel,
/* FP:extern_abi.rs-0075 */ 
/* FP:extern_abi.rs-0076 */     /* interrupt */
/* FP:extern_abi.rs-0077 */     AvrInterrupt,
/* FP:extern_abi.rs-0078 */     AvrNonBlockingInterrupt,
/* FP:extern_abi.rs-0079 */     Msp430Interrupt,
/* FP:extern_abi.rs-0080 */     RiscvInterruptM,
/* FP:extern_abi.rs-0081 */     RiscvInterruptS,
/* FP:extern_abi.rs-0082 */     X86Interrupt,
/* FP:extern_abi.rs-0083 */ 
/* FP:extern_abi.rs-0084 */     /* x86 */
/* FP:extern_abi.rs-0085 */     /// `ExternAbi::C` but spelled funny because x86
/* FP:extern_abi.rs-0086 */     Cdecl {
/* FP:extern_abi.rs-0087 */         unwind: bool,
/* FP:extern_abi.rs-0088 */     },
/* FP:extern_abi.rs-0089 */     /// gnu-stdcall on "unix" and win-stdcall on "windows"
/* FP:extern_abi.rs-0090 */     Stdcall {
/* FP:extern_abi.rs-0091 */         unwind: bool,
/* FP:extern_abi.rs-0092 */     },
/* FP:extern_abi.rs-0093 */     /// gnu-fastcall on "unix" and win-fastcall on "windows"
/* FP:extern_abi.rs-0094 */     Fastcall {
/* FP:extern_abi.rs-0095 */         unwind: bool,
/* FP:extern_abi.rs-0096 */     },
/* FP:extern_abi.rs-0097 */     /// windows C++ ABI
/* FP:extern_abi.rs-0098 */     Thiscall {
/* FP:extern_abi.rs-0099 */         unwind: bool,
/* FP:extern_abi.rs-0100 */     },
/* FP:extern_abi.rs-0101 */     /// uses AVX and stuff
/* FP:extern_abi.rs-0102 */     Vectorcall {
/* FP:extern_abi.rs-0103 */         unwind: bool,
/* FP:extern_abi.rs-0104 */     },
/* FP:extern_abi.rs-0105 */ 
/* FP:extern_abi.rs-0106 */     /* x86_64 */
/* FP:extern_abi.rs-0107 */     SysV64 {
/* FP:extern_abi.rs-0108 */         unwind: bool,
/* FP:extern_abi.rs-0109 */     },
/* FP:extern_abi.rs-0110 */     Win64 {
/* FP:extern_abi.rs-0111 */         unwind: bool,
/* FP:extern_abi.rs-0112 */     },
/* FP:extern_abi.rs-0113 */ }
/* FP:extern_abi.rs-0114 */ 
/* FP:extern_abi.rs-0115 */ macro_rules! abi_impls {
/* FP:extern_abi.rs-0116 */     ($e_name:ident = {
/* FP:extern_abi.rs-0117 */         $($variant:ident $({ unwind: $uw:literal })? =><= $tok:literal,)*
/* FP:extern_abi.rs-0118 */     }) => {
/* FP:extern_abi.rs-0119 */         impl $e_name {
/* FP:extern_abi.rs-0120 */             pub const ALL_VARIANTS: &[Self] = &[
/* FP:extern_abi.rs-0121 */                 $($e_name::$variant $({ unwind: $uw })*,)*
/* FP:extern_abi.rs-0122 */             ];
/* FP:extern_abi.rs-0123 */             pub const fn as_str(&self) -> &'static str {
/* FP:extern_abi.rs-0124 */                 match self {
/* FP:extern_abi.rs-0125 */                     $($e_name::$variant $( { unwind: $uw } )* => $tok,)*
/* FP:extern_abi.rs-0126 */                 }
/* FP:extern_abi.rs-0127 */             }
/* FP:extern_abi.rs-0128 */         }
/* FP:extern_abi.rs-0129 */ 
/* FP:extern_abi.rs-0130 */         impl ::core::str::FromStr for $e_name {
/* FP:extern_abi.rs-0131 */             type Err = AbiFromStrErr;
/* FP:extern_abi.rs-0132 */             fn from_str(s: &str) -> Result<$e_name, Self::Err> {
/* FP:extern_abi.rs-0133 */                 match s {
/* FP:extern_abi.rs-0134 */                     $($tok => Ok($e_name::$variant $({ unwind: $uw })*),)*
/* FP:extern_abi.rs-0135 */                     _ => Err(AbiFromStrErr::Unknown),
/* FP:extern_abi.rs-0136 */                 }
/* FP:extern_abi.rs-0137 */             }
/* FP:extern_abi.rs-0138 */         }
/* FP:extern_abi.rs-0139 */     }
/* FP:extern_abi.rs-0140 */ }
/* FP:extern_abi.rs-0141 */ 
/* FP:extern_abi.rs-0142 */ abi_impls! {
/* FP:extern_abi.rs-0143 */     ExternAbi = {
/* FP:extern_abi.rs-0144 */             C { unwind: false } =><= "C",
/* FP:extern_abi.rs-0145 */             C { unwind: true } =><= "C-unwind",
/* FP:extern_abi.rs-0146 */             Rust =><= "Rust",
/* FP:extern_abi.rs-0147 */             Aapcs { unwind: false } =><= "aapcs",
/* FP:extern_abi.rs-0148 */             Aapcs { unwind: true } =><= "aapcs-unwind",
/* FP:extern_abi.rs-0149 */             AvrInterrupt =><= "avr-interrupt",
/* FP:extern_abi.rs-0150 */             AvrNonBlockingInterrupt =><= "avr-non-blocking-interrupt",
/* FP:extern_abi.rs-0151 */             Cdecl { unwind: false } =><= "cdecl",
/* FP:extern_abi.rs-0152 */             Cdecl { unwind: true } =><= "cdecl-unwind",
/* FP:extern_abi.rs-0153 */             CmseNonSecureCall =><= "cmse-nonsecure-call",
/* FP:extern_abi.rs-0154 */             CmseNonSecureEntry =><= "cmse-nonsecure-entry",
/* FP:extern_abi.rs-0155 */             Custom =><= "custom",
/* FP:extern_abi.rs-0156 */             EfiApi =><= "efiapi",
/* FP:extern_abi.rs-0157 */             Fastcall { unwind: false } =><= "fastcall",
/* FP:extern_abi.rs-0158 */             Fastcall { unwind: true } =><= "fastcall-unwind",
/* FP:extern_abi.rs-0159 */             GpuKernel =><= "gpu-kernel",
/* FP:extern_abi.rs-0160 */             Msp430Interrupt =><= "msp430-interrupt",
/* FP:extern_abi.rs-0161 */             PtxKernel =><= "ptx-kernel",
/* FP:extern_abi.rs-0162 */             RiscvInterruptM =><= "riscv-interrupt-m",
/* FP:extern_abi.rs-0163 */             RiscvInterruptS =><= "riscv-interrupt-s",
/* FP:extern_abi.rs-0164 */             RustCall =><= "rust-call",
/* FP:extern_abi.rs-0165 */             RustCold =><= "rust-cold",
/* FP:extern_abi.rs-0166 */             RustInvalid =><= "rust-invalid",
/* FP:extern_abi.rs-0167 */             Stdcall { unwind: false } =><= "stdcall",
/* FP:extern_abi.rs-0168 */             Stdcall { unwind: true } =><= "stdcall-unwind",
/* FP:extern_abi.rs-0169 */             System { unwind: false } =><= "system",
/* FP:extern_abi.rs-0170 */             System { unwind: true } =><= "system-unwind",
/* FP:extern_abi.rs-0171 */             SysV64 { unwind: false } =><= "sysv64",
/* FP:extern_abi.rs-0172 */             SysV64 { unwind: true } =><= "sysv64-unwind",
/* FP:extern_abi.rs-0173 */             Thiscall { unwind: false } =><= "thiscall",
/* FP:extern_abi.rs-0174 */             Thiscall { unwind: true } =><= "thiscall-unwind",
/* FP:extern_abi.rs-0175 */             Unadjusted =><= "unadjusted",
/* FP:extern_abi.rs-0176 */             Vectorcall { unwind: false } =><= "vectorcall",
/* FP:extern_abi.rs-0177 */             Vectorcall { unwind: true } =><= "vectorcall-unwind",
/* FP:extern_abi.rs-0178 */             Win64 { unwind: false } =><= "win64",
/* FP:extern_abi.rs-0179 */             Win64 { unwind: true } =><= "win64-unwind",
/* FP:extern_abi.rs-0180 */             X86Interrupt =><= "x86-interrupt",
/* FP:extern_abi.rs-0181 */     }
/* FP:extern_abi.rs-0182 */ }
/* FP:extern_abi.rs-0183 */ 
/* FP:extern_abi.rs-0184 */ impl Ord for ExternAbi {
/* FP:extern_abi.rs-0185 */     fn cmp(&self, rhs: &Self) -> Ordering {
/* FP:extern_abi.rs-0186 */         self.as_str().cmp(rhs.as_str())
/* FP:extern_abi.rs-0187 */     }
/* FP:extern_abi.rs-0188 */ }
/* FP:extern_abi.rs-0189 */ 
/* FP:extern_abi.rs-0190 */ impl PartialOrd for ExternAbi {
/* FP:extern_abi.rs-0191 */     fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
/* FP:extern_abi.rs-0192 */         Some(self.cmp(rhs))
/* FP:extern_abi.rs-0193 */     }
/* FP:extern_abi.rs-0194 */ }
/* FP:extern_abi.rs-0195 */ 
/* FP:extern_abi.rs-0196 */ impl PartialEq for ExternAbi {
/* FP:extern_abi.rs-0197 */     fn eq(&self, rhs: &Self) -> bool {
/* FP:extern_abi.rs-0198 */         self.cmp(rhs) == Ordering::Equal
/* FP:extern_abi.rs-0199 */     }
/* FP:extern_abi.rs-0200 */ }
/* FP:extern_abi.rs-0201 */ 
/* FP:extern_abi.rs-0202 */ impl Eq for ExternAbi {}
/* FP:extern_abi.rs-0203 */ 
/* FP:extern_abi.rs-0204 */ impl Hash for ExternAbi {
/* FP:extern_abi.rs-0205 */     fn hash<H: Hasher>(&self, state: &mut H) {
/* FP:extern_abi.rs-0206 */         self.as_str().hash(state);
/* FP:extern_abi.rs-0207 */         // double-assurance of a prefix breaker
/* FP:extern_abi.rs-0208 */         u32::from_be_bytes(*b"ABI\0").hash(state);
/* FP:extern_abi.rs-0209 */     }
/* FP:extern_abi.rs-0210 */ }
/* FP:extern_abi.rs-0211 */ 
/* FP:extern_abi.rs-0212 */ #[cfg(feature = "nightly")]
/* FP:extern_abi.rs-0213 */ impl<C> HashStable<C> for ExternAbi {
/* FP:extern_abi.rs-0214 */     #[inline]
/* FP:extern_abi.rs-0215 */     fn hash_stable(&self, _: &mut C, hasher: &mut StableHasher) {
/* FP:extern_abi.rs-0216 */         Hash::hash(self, hasher);
/* FP:extern_abi.rs-0217 */     }
/* FP:extern_abi.rs-0218 */ }
/* FP:extern_abi.rs-0219 */ 
/* FP:extern_abi.rs-0220 */ #[cfg(feature = "nightly")]
/* FP:extern_abi.rs-0221 */ impl StableOrd for ExternAbi {
/* FP:extern_abi.rs-0222 */     const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:extern_abi.rs-0223 */ 
/* FP:extern_abi.rs-0224 */     // because each ABI is hashed like a string, there is no possible instability
/* FP:extern_abi.rs-0225 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:extern_abi.rs-0226 */ }
/* FP:extern_abi.rs-0227 */ 
/* FP:extern_abi.rs-0228 */ #[cfg(feature = "nightly")]
/* FP:extern_abi.rs-0229 */ crate::rustc_error_messages::into_diag_arg_using_display!(ExternAbi);
/* FP:extern_abi.rs-0230 */ 
/* FP:extern_abi.rs-0231 */ #[cfg(feature = "nightly")]
/* FP:extern_abi.rs-0232 */ pub enum CVariadicStatus {
/* FP:extern_abi.rs-0233 */     NotSupported,
/* FP:extern_abi.rs-0234 */     Stable,
/* FP:extern_abi.rs-0235 */     Unstable { feature: Symbol },
/* FP:extern_abi.rs-0236 */ }
/* FP:extern_abi.rs-0237 */ 
/* FP:extern_abi.rs-0238 */ impl ExternAbi {
/* FP:extern_abi.rs-0239 */     /// An ABI "like Rust"
/* FP:extern_abi.rs-0240 */     ///
/* FP:extern_abi.rs-0241 */     /// These ABIs are fully controlled by the Rust compiler, which means they
/* FP:extern_abi.rs-0242 */     /// - support unwinding with `-Cpanic=unwind`, unlike `extern "C"`
/* FP:extern_abi.rs-0243 */     /// - often diverge from the C ABI
/* FP:extern_abi.rs-0244 */     /// - are subject to change between compiler versions
/* FP:extern_abi.rs-0245 */     pub fn is_rustic_abi(self) -> bool {
/* FP:extern_abi.rs-0246 */         use ExternAbi::*;
/* FP:extern_abi.rs-0247 */         matches!(self, Rust | RustCall | RustCold)
/* FP:extern_abi.rs-0248 */     }
/* FP:extern_abi.rs-0249 */ 
/* FP:extern_abi.rs-0250 */     /// Returns whether the ABI supports C variadics. This only controls whether we allow *imports*
/* FP:extern_abi.rs-0251 */     /// of such functions via `extern` blocks; there's a separate check during AST construction
/* FP:extern_abi.rs-0252 */     /// guarding *definitions* of variadic functions.
/* FP:extern_abi.rs-0253 */     #[cfg(feature = "nightly")]
/* FP:extern_abi.rs-0254 */     pub fn supports_c_variadic(self) -> CVariadicStatus {
/* FP:extern_abi.rs-0255 */         // * C and Cdecl obviously support varargs.
/* FP:extern_abi.rs-0256 */         // * C can be based on Aapcs, SysV64 or Win64, so they must support varargs.
/* FP:extern_abi.rs-0257 */         // * EfiApi is based on Win64 or C, so it also supports it.
/* FP:extern_abi.rs-0258 */         // * System automatically falls back to C when used with variadics, therefore supports it.
/* FP:extern_abi.rs-0259 */         //
/* FP:extern_abi.rs-0260 */         // * Stdcall does not, because it would be impossible for the callee to clean
/* FP:extern_abi.rs-0261 */         //   up the arguments. (callee doesn't know how many arguments are there)
/* FP:extern_abi.rs-0262 */         // * Same for Fastcall, Vectorcall and Thiscall.
/* FP:extern_abi.rs-0263 */         // * Other calling conventions are related to hardware or the compiler itself.
/* FP:extern_abi.rs-0264 */         //
/* FP:extern_abi.rs-0265 */         // All of the supported ones must have a test in `tests/codegen/cffi/c-variadic-ffi.rs`.
/* FP:extern_abi.rs-0266 */         match self {
/* FP:extern_abi.rs-0267 */             Self::C { .. }
/* FP:extern_abi.rs-0268 */             | Self::Cdecl { .. }
/* FP:extern_abi.rs-0269 */             | Self::Aapcs { .. }
/* FP:extern_abi.rs-0270 */             | Self::Win64 { .. }
/* FP:extern_abi.rs-0271 */             | Self::SysV64 { .. }
/* FP:extern_abi.rs-0272 */             | Self::EfiApi => CVariadicStatus::Stable,
/* FP:extern_abi.rs-0273 */             Self::System { .. } => {
/* FP:extern_abi.rs-0274 */                 CVariadicStatus::Unstable { feature: crate::rustc_span::sym::extern_system_varargs }
/* FP:extern_abi.rs-0275 */             }
/* FP:extern_abi.rs-0276 */             _ => CVariadicStatus::NotSupported,
/* FP:extern_abi.rs-0277 */         }
/* FP:extern_abi.rs-0278 */     }
/* FP:extern_abi.rs-0279 */ }
/* FP:extern_abi.rs-0280 */ 
/* FP:extern_abi.rs-0281 */ pub fn all_names() -> Vec<&'static str> {
/* FP:extern_abi.rs-0282 */     ExternAbi::ALL_VARIANTS.iter().map(|abi| abi.as_str()).collect()
/* FP:extern_abi.rs-0283 */ }
/* FP:extern_abi.rs-0284 */ 
/* FP:extern_abi.rs-0285 */ impl ExternAbi {
/* FP:extern_abi.rs-0286 */     /// Default ABI chosen for `extern fn` declarations without an explicit ABI.
/* FP:extern_abi.rs-0287 */     pub const FALLBACK: ExternAbi = ExternAbi::C { unwind: false };
/* FP:extern_abi.rs-0288 */ 
/* FP:extern_abi.rs-0289 */     pub fn name(self) -> &'static str {
/* FP:extern_abi.rs-0290 */         self.as_str()
/* FP:extern_abi.rs-0291 */     }
/* FP:extern_abi.rs-0292 */ }
/* FP:extern_abi.rs-0293 */ 
/* FP:extern_abi.rs-0294 */ impl fmt::Display for ExternAbi {
/* FP:extern_abi.rs-0295 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:extern_abi.rs-0296 */         write!(f, "\"{}\"", self.as_str())
/* FP:extern_abi.rs-0297 */     }
/* FP:extern_abi.rs-0298 */ }