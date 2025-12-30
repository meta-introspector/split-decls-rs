// Generated macro for impl_508 (impl)
macro_rules! Depcrate_options_coreimpl_508 {
() => {
// Module: crate::options::core
// Provides: {"impl_508"}
// Dependencies: {}
impl Core { # [doc = " Partially initializes `Core` by reading the identity, generics, and body shape."] pub fn start (di : & syn :: DeriveInput) -> Result < Self > { Ok (Core { ident : di . ident . clone () , generics : di . generics . clone () , data : Data :: try_empty_from (& di . data) ? , default : Default :: default () , rename_rule : if let syn :: Data :: Enum (_) = di . data { RenameRule :: SnakeCase } else { Default :: default () } , post_transform : Default :: default () , bound : Default :: default () , allow_unknown_fields : Default :: default () , }) } fn as_codegen_default (& self) -> Option < codegen :: DefaultExpression < '_ > > { self . default . as_ref () . map (| expr | match * expr { DefaultExpression :: Explicit (ref callable) => { codegen :: DefaultExpression :: Explicit (callable) } DefaultExpression :: Inherit => { panic ! ("DefaultExpression::Inherit is not valid at container level") } DefaultExpression :: Trait { span } => codegen :: DefaultExpression :: Trait { span } , }) } }
};
}
