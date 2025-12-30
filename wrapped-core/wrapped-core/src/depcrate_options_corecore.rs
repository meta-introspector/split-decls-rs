// Generated macro for Core (struct)
macro_rules! Depcrate_options_coreCore {
() => {
// Module: crate::options::core
// Provides: {"Core"}
// Dependencies: {}
# [doc = " A struct or enum which should have `FromMeta` or `FromDeriveInput` implementations"] # [doc = " generated."] # [derive (Debug , Clone)] pub struct Core { # [doc = " The type identifier."] pub ident : syn :: Ident , # [doc = " The type's generics. If the type does not use any generics, this will"] # [doc = " be an empty instance."] pub generics : syn :: Generics , # [doc = " Controls whether missing properties should cause errors or should be filled by"] # [doc = " the result of a function call. This can be overridden at the field level."] pub default : Option < DefaultExpression > , # [doc = " The rule that should be used to rename all fields/variants in the container."] pub rename_rule : RenameRule , # [doc = " A transform which will be called on `darling::Result<Self>`. It must either be"] # [doc = " an `FnOnce(T) -> T` when `map` is used, or `FnOnce(T) -> darling::Result<T>` when"] # [doc = " `and_then` is used."] # [doc = ""] # [doc = " `map` and `and_then` are mutually-exclusive to avoid confusion about the order in"] # [doc = " which the two are applied."] pub post_transform : Option < codegen :: PostfixTransform > , # [doc = " The body of the _deriving_ type."] pub data : Data < InputVariant , InputField > , # [doc = " The custom bound to apply to the generated impl"] pub bound : Option < Vec < syn :: WherePredicate > > , # [doc = " Whether or not unknown fields should produce an error at compilation time."] pub allow_unknown_fields : Option < bool > , }
};
}
