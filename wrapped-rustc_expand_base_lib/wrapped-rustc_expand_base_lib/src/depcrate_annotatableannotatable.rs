// Generated macro for Annotatable (enum)
macro_rules! Depcrate_annotatableAnnotatable {
() => {
// Module: crate::annotatable
// Provides: {"Annotatable"}
// Dependencies: {}
# [doc = " Represents an Abstract Syntax Tree (AST) node that can be annotated with attributes."] # [doc = ""] # [doc = " This enum provides a unified way to handle different kinds of AST nodes (items, expressions,"] # [doc = " statements, etc.) that can be targets for attributes in macro expansion. When a macro"] # [doc = " expands, it often receives or produces `Annotatable` types."] # [doc = ""] # [doc = " For n00bs: Think of `Annotatable` as a \"wrapper\" that can hold many different kinds of Rust"] # [doc = " code structures (like a function, a variable, an enum, etc.). The macro system uses this"] # [doc = " wrapper so it doesn't have to write separate code for every single type of Rust code it"] # [doc = " might want to change or look at."] # [doc = ""] # [doc = " When adding new variants here, remember that other parts of the macro system,"] # [doc = " particularly those that visit or transform AST nodes (like `InvocationCollector`),"] # [doc = " might need to be updated to handle the new variant correctly."] # [derive (Debug , Clone)] pub enum Annotatable { Item (Box < ast :: Item >) , AssocItem (Box < ast :: AssocItem > , AssocCtxt) , ForeignItem (Box < ast :: ForeignItem >) , Stmt (Box < ast :: Stmt >) , Expr (Box < ast :: Expr >) , Ty (Box < ast :: Ty >) , Pat (Box < ast :: Pat >) , Arm (ast :: Arm) , ExprField (ast :: ExprField) , PatField (ast :: PatField) , GenericParam (ast :: GenericParam) , Param (ast :: Param) , FieldDef (ast :: FieldDef) , Variant (ast :: Variant) , WherePredicate (ast :: WherePredicate) , Crate (ast :: Crate) , }
};
}
