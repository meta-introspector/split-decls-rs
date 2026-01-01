// wrap_types.rs - Dynamically generated support types

pub mod type_names {
    pub struct {compute_debuginfo_type_name;
    pub struct compute_debuginfo_vtable_name};
}
pub mod save {
    pub struct build_dep_graph;
}
pub mod erase {
    pub struct EraseType;
}
pub mod path {
    pub struct PathStyle;
}
pub mod fuchsia {
    pub struct *;
}
pub mod write {
    pub struct WriteBackendMethods;
}
pub mod waker {
    pub struct SyncWaker;
    pub struct current_thread_id;
    pub struct Waker;
}
pub mod constraints {
    pub struct *;
}
pub mod emit {
    pub struct address_for_func;
}
pub mod normalize {
    pub struct normalize_with_depth_to;
}
pub mod region_constraints {
    pub struct RegionSnapshot;
    pub struct {RegionConstraintData;
    pub struct UndoLog};
}
pub mod namespace {
    pub struct mangled_name_of_instance;
    pub struct item_namespace;
}
pub mod consts {
    pub struct ConstCodegenMethods;
}
pub mod test_result {
    pub struct TestResult;
}
pub mod graph {
    pub struct {self;
    pub struct BasicCoverageBlock};
    pub struct {CurrentDepGraph;
    pub struct DepNodeColor;
    pub struct DepNodeColorMap};
}
pub mod diagnostics {
    pub struct {;
    pub struct {FailedMacro;
    pub struct failed_to_match_macro};
    pub struct {ConsumeClosingDelim;
    pub struct dummy_arg};
    pub struct SnapshotParser;
    pub struct AttemptLocalParseRecovery;
}
pub mod cli {
    pub struct TestOpts;
    pub struct Language;
    pub struct FailureReason;
}
pub mod errors {
    pub struct {;
    pub struct {InvalidAbi;
    pub struct InvalidAbiSuggestion;
    pub struct TupleStructWithDefault;
    pub struct UnionWithDefault};
    pub struct GenericsArgsErrExtend;
}
pub mod region_ctxt {
    pub struct RegionCtxt;
}
pub mod place {
    pub struct {PlaceRef;
    pub struct PlaceValue;
    pub struct codegen_tag_value};
    pub struct PlaceRef;
    pub struct {PlaceRef;
    pub struct PlaceValue};
}
pub mod suggestions {
    pub struct get_explanation_based_on_obligation;
}
pub mod event {
    pub struct {CompletedTest;
    pub struct TestEvent};
    pub struct CompletedTest;
}
pub mod argument {
    pub struct Argument;
    pub struct ArgumentList;
}
pub mod data {
    pub struct *;
}
pub mod search {
    pub struct SearchResult::*;
    pub struct SearchBound;
}
pub mod compiler_interface {
    pub struct BridgeTys;
}
pub mod item_bounds {
    pub struct explicit_item_bounds_with_filter;
}
pub mod simplify {
    pub struct simplify_cfg;
}
pub mod pat {
    pub struct Expected;
    pub struct {CommaRecoveryMode;
    pub struct Expected;
    pub struct RecoverColon;
    pub struct RecoverComma};
    pub struct {PatternLocation;
    pub struct RecoverComma};
}
pub mod sqrt {
    pub struct sqrt;
    pub struct sqrtf;
}
pub mod reverse_sccs {
    pub struct ReverseSccGraph;
}
pub mod common {
    pub struct *;
}
pub mod options {
    pub struct {Options;
    pub struct OutputFormat};
    pub struct BenchMode;
    pub struct {ColorConfig;
    pub struct Options;
    pub struct OutputFormat;
    pub struct RunIgnored};
    pub struct ShouldPanic;
}
pub mod ops {
    pub struct {self;
    pub struct NonConstOp;
    pub struct Status};
}
pub mod method {
    pub struct probe;
    pub struct MethodCallee;
    pub struct probe::ProbeScope;
}
pub mod coverageinfo {
    pub struct CoverageInfoBuilderMethods;
}
pub mod super {
    pub struct *;
    pub struct tests::TestGraph;
    pub struct riscv::imply_features;
    pub struct backtrace::miri::{Frame;
    pub struct resolve_addr};
    pub struct super::generic;
    pub struct {;
    pub struct navigate;
    pub struct {dbghelp;
    pub struct windows_sys::*};
    pub struct is_cyclic;
    pub struct mem::{is_enclave_range;
    pub struct is_user_range};
    pub struct borrow::DormantMutRef;
    pub struct node::{Handle;
    pub struct NodeRef;
    pub struct marker};
    pub struct cvt_nz;
    pub struct windows_prefix::*;
    pub struct super::windows_sys::*;
    pub struct helpers;
    pub struct windows_sys::*;
}
pub mod trait_goals {
    pub struct TraitGoalProvenVia;
}
pub mod fs {
    pub struct *;
}
pub mod ty {
    pub struct {Allocation;
    pub struct ProvenanceMap};
    pub struct {AllowPlus;
    pub struct RecoverQPath;
    pub struct RecoverReturnSign};
    pub struct {;
}
pub mod alloc {
    pub struct {User;
    pub struct copy_from_userspace;
    pub struct copy_to_userspace};
    pub struct *;
}
pub mod compare_impl_item {
    pub struct check_type_bounds;
}
pub mod key {
    pub struct {Key;
    pub struct LazyKey;
    pub struct get;
    pub struct set};
}
pub mod callee {
    pub struct DeferredCallResolution;
}
pub mod lto {
    pub struct {self;
    pub struct SerializedModule};
}
pub mod type_ {
    pub struct {ArgAbiBuilderMethods;
    pub struct BaseTypeCodegenMethods;
    pub struct LayoutTypeCodegenMethods};
}
pub mod query {
    pub struct DepGraphQuery;
}
pub mod api {
    pub struct {self;
    pub struct WinError};
}
pub mod usercalls {
    pub struct alloc::UserRef;
}
pub mod abi {
    pub struct usercalls;
    pub struct AbiBuilderMethods;
    pub struct ReprOptions;
}
pub mod linker {
    pub struct {self;
    pub struct Linker};
}
pub mod metadata {
    pub struct {MetadataPosition;
    pub struct create_wrapper_file};
    pub struct {create_compressed_metadata_file;
    pub struct search_for_section};
    pub struct file_metadata;
}
pub mod thread_info {
    pub struct {delete_current_info;
    pub struct set_current_info;
    pub struct with_current_info};
}
pub mod pattern {
    pub struct {DoubleEndedSearcher;
    pub struct Pattern;
    pub struct ReverseSearcher;
    pub struct Searcher};
}
pub mod debug {
    pub struct EdgeFilter;
    pub struct std::env};
}
pub mod validations {
    pub struct {next_code_point;
    pub struct next_code_point_reverse};
    pub struct run_utf8_validation;
    pub struct utf8_char_width;
}
pub mod formatters {
    pub struct {;
}
pub mod corpora {
    pub struct *;
}
pub mod UndefinedBehaviorInfo {
    pub struct *;
}
pub mod coherence {
    pub struct {self;
    pub struct Conflict};
}
pub mod detect {
    pub struct {cpu_flags;
    pub struct get_cpu_features};
}
pub mod probe {
    pub struct {AutorefOrPtrAdjustment;
    pub struct IsSuggestion;
    pub struct Mode;
    pub struct ProbeScope};
}
pub mod pretty {
    pub struct dump_mir_def_ids;
}
pub mod node {
    pub struct {self;
    pub struct Root};
    pub struct ForceResult::*;
    pub struct Root;
    pub struct LeftOrRight::*;
    pub struct {Handle;
    pub struct NodeRef;
    pub struct Root;
    pub struct marker};
    pub struct {Handle;
    pub struct NodeRef;
    pub struct marker};
}
pub mod rpath {
    pub struct {self;
    pub struct RPathConfig};
}
pub mod Expectation {
    pub struct *;
}
pub mod serialized {
    pub struct {GraphEncoder;
    pub struct SerializedDepGraph;
    pub struct SerializedDepNodeIndex};
}
pub mod command {
    pub struct Command;
}
pub mod neon {
    pub struct *;
}
pub mod helpers {
    pub struct concurrency::get_concurrency;
    pub struct metrics::MetricMap;
}
pub mod explain_borrow {
    pub struct {BorrowExplanation;
    pub struct LaterUseKind};
}
pub mod stability {
    pub struct {enabled_names;
    pub struct gate_unstable_abi};
}
pub mod delegate {
    pub struct SolverDelegate;
}
pub mod values {
    pub struct value_for_array;
}
pub mod display_buffer {
    pub struct DisplayBuffer;
}
pub mod attr {
    pub struct InnerAttrForbiddenReason;
}
pub mod select {
    pub struct {Operation;
    pub struct Selected;
    pub struct Token};
    pub struct SelectionContext;
    pub struct Selected;
    pub struct {Operation;
    pub struct Selected};
}
pub mod operand {
    pub struct {OperandRef;
    pub struct OperandRefBuilder;
    pub struct OperandValue};
    pub struct OperandValue;
    pub struct OperandRef;
    pub struct OperandValue::{Immediate;
    pub struct Pair;
    pub struct Ref;
    pub struct ZeroSized};
    pub struct {OperandRef;
    pub struct OperandValue};
}
pub mod interpret {
    pub struct ReportedErrorInfo;
    pub struct GlobalAlloc;
}
pub mod raw {
    pub struct {AsRawSocket;
    pub struct FromRawSocket;
    pub struct IntoRawSocket;
    pub struct RawSocket};
    pub struct {AsRawHandle;
    pub struct FromRawHandle;
    pub struct IntoRawHandle;
    pub struct RawHandle};
    pub struct {AsRawFd;
    pub struct FromRawFd;
    pub struct IntoRawFd;
    pub struct RawFd};
}
pub mod auxvec {
    pub struct auxv_from_file;
}
pub mod support {
    pub struct Round;
    pub struct Float;
    pub struct {Float;
    pub struct Round};
}
pub mod effects {
    pub struct {self;
    pub struct HostEffectObligation};
}
pub mod nice_region_error {
    pub struct placeholder_error::Highlighted;
    pub struct find_anon_type;
}
pub mod intrinsic {
    pub struct ArmIntrinsicType;
    pub struct IntrinsicCallBuilderMethods;
    pub struct {IntrinsicDefinition;
    pub struct format_f16_return_value};
    pub struct IntrinsicDefinition;
}
pub mod ErrorKind {
    pub struct *;
}
pub mod sealed {
    pub struct Sealed;
}
pub mod UnsupportedOpInfo {
    pub struct *;
}
pub mod project {
    pub struct {self;
    pub struct ProjectAndUnifyResult};
    pub struct ProjectionTermObligation;
}
pub mod mystd {
    pub struct ffi::OsStr;
    pub struct os::unix::prelude::*;
    pub struct os::unix::ffi::OsStrExt;
    pub struct path::Path;
    pub struct ffi::OsString;
    pub struct fs::File;
    pub struct io::Read;
    pub struct io::{Read;
    pub struct Seek;
    pub struct SeekFrom};
    pub struct fs;
    pub struct path::{Path;
    pub struct PathBuf};
    pub struct env;
    pub struct os::windows::prelude::*;
    pub struct prelude::v1::*;
    pub struct ffi::{OsStr;
    pub struct OsString};
    pub struct os::unix::ffi::OsStringExt;
}
pub mod assembly {
    pub struct {Candidate;
    pub struct structural_traits};
}
pub mod qualifs {
    pub struct {self;
    pub struct HasMutInterior;
    pub struct NeedsDrop;
    pub struct NeedsNonConstDrop};
}
pub mod InterpErrorKind {
    pub struct UndefinedBehavior as Ub;
    pub struct Unsupported as Unsup;
}
pub mod matches {
    pub struct BuiltMatchTree;
}
pub mod time {
    pub struct with_tmos;
    pub struct {TestExecTime;
    pub struct TestSuiteExecTime};
    pub struct Timespec;
    pub struct system_time_internal::{from_uefi;
    pub struct to_uefi};
    pub struct TestTimeOptions;
    pub struct TestExecTime;
}
pub mod backtrace {
    pub struct Frame;
}
pub mod merge_iter {
    pub struct MergeIterInner;
}
pub mod layout_test {
    pub struct ensure_wf;
}
pub mod Entry {
    pub struct {Occupied;
    pub struct Vacant};
}
pub mod bench {
    pub struct fmt_bench_samples;
    pub struct BenchSamples;
    pub struct Bencher;
}
pub mod map {
    pub struct MIN_LEN;
    pub struct SsoHashMap;
}
pub mod intrinsic_helpers {
    pub struct IntrinsicTypeDefinition;
    pub struct {IntrinsicTypeDefinition;
    pub struct TypeKind};
}
pub mod fmt {
    pub struct {Formatted;
    pub struct Part};
    pub struct {DebugDiffWithAdapter;
    pub struct DebugWithAdapter;
    pub struct DebugWithContext};
}
pub mod mir {
    pub struct Mutability;
    pub struct {Body;
    pub struct Mutability;
    pub struct Safety};
}
pub mod predicates_of {
    pub struct assert_only_contains_predicates_from;
}
pub mod generic_graph {
    pub struct mir_fn_to_generic_graph;
}
pub mod utils {
    pub struct {Backoff;
    pub struct CachePadded};
    pub struct *;
    pub struct {DIB;
    pub struct debug_context};
    pub struct SubdiagnosticVariant;
    pub struct {;
    pub struct DIB;
    pub struct Backoff;
}
pub mod explicit {
    pub struct ExplicitPredicatesMap;
}
pub mod dwarf {
    pub struct eh::{self;
    pub struct EHAction;
    pub struct EHContext};
}
pub mod outlives {
    pub struct test_type_match;
}
pub mod misc {
    pub struct MiscCodegenMethods;
}
pub mod indentation {
    pub struct Indentation;
}
pub mod hermit_abi {
    pub struct {self;
    pub struct CLOCK_MONOTONIC;
    pub struct CLOCK_REALTIME;
    pub struct timespec};
}
pub mod link {
    pub struct {self;
    pub struct ensure_removed};
}
pub mod macro_parser {
    pub struct {NamedMatches;
    pub struct NamedParseResult};
}
pub mod resolver {
    pub struct FlowSensitiveAnalysis;
}
pub mod context {
    pub struct Context;
    pub struct CompilerCtxt;
}
pub mod memory {
    pub struct MemoryKind;
}
pub mod macro_rules {
    pub struct {MacroRule;
    pub struct NoopTracker;
    pub struct parser_from_cx};
}
pub mod object {
    pub struct WriteDebugInfo;
}
pub mod type_map {
    pub struct {DINodeCreationResult;
    pub struct UniqueTypeId};
}
pub mod public_extern {
    pub struct *;
}
pub mod types {
    pub struct {NamePadding;
    pub struct TestDesc;
    pub struct TestDescAndFn};
    pub struct *;
    pub struct {TestDesc;
    pub struct TestId};
    pub struct {TestDesc;
    pub struct TestType};
    pub struct BytesOrWideString;
    pub struct TestDesc;
}
pub mod inspect {
    pub struct {self;
    pub struct ProofTreeInferCtxtExt};
}
pub mod rvalue {
    pub struct transmute_scalar;
}
pub mod archive {
    pub struct {ArchiveBuilder;
    pub struct ArchiveBuilderBuilder};
}
pub mod visitor {
    pub struct ResultsVisitor;
}
pub mod eval_queries {
    pub struct {mk_eval_cx_to_read_const_val;
    pub struct op_to_const};
}
pub mod lattice {
    pub struct MaybeReachable;
}
pub mod borrow_set {
    pub struct BorrowData;
}
pub mod move_paths {
    pub struct {InitKind;
    pub struct LookupResult;
    pub struct MoveData;
    pub struct MovePathIndex};
}
pub mod query_context {
    pub struct test::{Def;
    pub struct UltraMinimal};
}
pub mod on_unimplemented {
    pub struct {AppendConstMessage;
    pub struct OnUnimplementedNote};
}
pub mod machine {
    pub struct CompileTimeInterpCx;
    pub struct AllocMap;
}
pub mod debuginfo {
    pub struct {;
    pub struct DebugInfoBuilderMethods;
}
pub mod util {
    pub struct ensure_monomorphic_enough;
    pub struct parse_single_integer;
    pub struct parse_version;
    pub struct closure_trait_ref_and_return_type;
}
pub mod fxhash {
    pub struct FxHashMap;
}
pub mod ffi {
    pub struct {AttributeKind;
    pub struct BasicBlock;
    pub struct Metadata;
    pub struct Module;
    pub struct Type;
    pub struct Value};
}
pub mod terms {
    pub struct VarianceTerm::*;
    pub struct *;
}
pub mod env {
    pub struct {CommandEnv;
    pub struct CommandEnvs};
    pub struct OutlivesEnvironment;
}
pub mod LabelText {
    pub struct {self;
    pub struct EscStr;
    pub struct HtmlStr;
    pub struct LabelStr};
}
pub mod combine {
    pub struct PredicateEmittingRelation;
}
pub mod unstable {
    pub struct Stable;
}
pub mod SelectionCandidate {
    pub struct {self;
    pub struct *};
    pub struct *;
}
pub mod windows_sys {
    pub struct *;
}
pub mod asm {
    pub struct AsmBuilderMethods;
}
pub mod rpc {
    pub struct {DecodeMut;
    pub struct Encode;
    pub struct Reader;
    pub struct Writer};
}
pub mod graphviz {
    pub struct write_mir_fn_graphviz;
}
pub mod error {
    pub struct *;
    pub struct expect_success_aborting;
    pub struct invalid_attr;
    pub struct {ItronError;
    pub struct fail;
    pub struct fail_aborting};
    pub struct expect_success;
}
pub mod constraint {
    pub struct Constraint;
}
