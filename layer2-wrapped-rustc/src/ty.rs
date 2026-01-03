// ty module stub
pub struct Ty;
pub struct TyCtxt;
pub struct TyKind;
pub struct RegionVid;
pub struct ParamEnvAnd;
pub struct Visibility;
pub struct Feed;
pub struct TyCtxtFeed;
pub struct MainDefinition;
pub struct RegisteredTools;
pub struct ResolverAstLowering;
pub struct ResolverGlobalCtxt;
pub struct DelegationFnSig;
pub struct ExistentialTraitRef;
pub struct TypeVisitableExt;

pub mod adjustment {}
pub mod print {
    pub fn with_no_trimmed_paths() {}
}
pub mod error {
    pub struct TypeError;
}
pub mod layout {
    pub struct LayoutOf;
    pub struct TyAndLayout;
    pub struct HasTyCtxt;
    pub struct HasTypingEnv;
    pub struct FnAbiOf;
}
pub mod fast_reject {}
