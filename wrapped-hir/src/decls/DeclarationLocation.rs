macro_rules! DeclarationLocation {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct DeclarationLocation { # [doc = " The file id for both the `ptr` and `name_ptr`."] pub hir_file_id : HirFileId , # [doc = " This points to the whole syntax node of the declaration."] pub ptr : SyntaxNodePtr , # [doc = " This points to the [`syntax::ast::Name`] identifier of the declaration."] pub name_ptr : AstPtr < Either < syntax :: ast :: Name , syntax :: ast :: NameRef > > , }
    };
}

DeclarationLocation!();