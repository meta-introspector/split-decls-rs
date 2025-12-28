macro_rules! AstDeclarationVisitor {
    () => {
        # [doc = " A `syn::visit::Visit` implementation to extract `Declaration`s from a `syn::File`."] # [derive (Debug , Default)] struct AstDeclarationVisitor { declarations : Vec < Declaration > , current_file_path : PathBuf , }
    };
}

AstDeclarationVisitor!();