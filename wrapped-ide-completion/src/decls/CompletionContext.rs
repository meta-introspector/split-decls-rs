macro_rules! deps {
    () => {
        AutoImportExclusionType!();
        CompletionConfig!();
        QualifierCtx!();
        CompleteSemicolon!();
    };
}

macro_rules! CompletionContext {
    () => {
        deps!();
        # [doc = " `CompletionContext` is created early during completion to figure out, where"] # [doc = " exactly is the cursor, syntax-wise."] # [derive (Debug)] pub (crate) struct CompletionContext < 'a > { pub (crate) sema : Semantics < 'a , RootDatabase > , pub (crate) scope : SemanticsScope < 'a > , pub (crate) db : & 'a RootDatabase , pub (crate) config : & 'a CompletionConfig < 'a > , pub (crate) position : FilePosition , pub (crate) trigger_character : Option < char > , # [doc = " The token before the cursor, in the original file."] pub (crate) original_token : SyntaxToken , # [doc = " The token before the cursor, in the macro-expanded file."] pub (crate) token : SyntaxToken , # [doc = " The crate of the current file."] pub (crate) krate : hir :: Crate , pub (crate) display_target : DisplayTarget , # [doc = " The module of the `scope`."] pub (crate) module : hir :: Module , # [doc = " The function where we're completing, if inside a function."] pub (crate) containing_function : Option < hir :: Function > , # [doc = " Whether nightly toolchain is used. Cached since this is looked up a lot."] pub (crate) is_nightly : bool , # [doc = " The edition of the current crate"] pub (crate) edition : Edition , # [doc = " The expected name of what we are completing."] # [doc = " This is usually the parameter name of the function argument we are completing."] pub (crate) expected_name : Option < NameOrNameRef > , # [doc = " The expected type of what we are completing."] pub (crate) expected_type : Option < Type < 'a > > , pub (crate) qualifier_ctx : QualifierCtx , pub (crate) locals : FxHashMap < Name , Local > , # [doc = " The module depth of the current module of the cursor position."] # [doc = " - crate-root"] # [doc = "  - mod foo"] # [doc = "   - mod bar"] # [doc = ""] # [doc = " Here depth will be 2"] pub (crate) depth_from_crate_root : usize , # [doc = " Traits whose methods will be excluded from flyimport. Flyimport should not suggest"] # [doc = " importing those traits."] # [doc = ""] # [doc = " Note the trait *themselves* are not excluded, only their methods are."] pub (crate) exclude_flyimport : FxHashMap < ModuleDef , AutoImportExclusionType > , # [doc = " Traits whose methods should always be excluded, even when in scope (compare `exclude_flyimport_traits`)."] # [doc = " They will *not* be excluded, however, if they are available as a generic bound."] # [doc = ""] # [doc = " Note the trait *themselves* are not excluded, only their methods are."] pub (crate) exclude_traits : FxHashSet < hir :: Trait > , # [doc = " Whether and how to complete semicolon for unit-returning functions."] pub (crate) complete_semicolon : CompleteSemicolon , }
    };
}

CompletionContext!()