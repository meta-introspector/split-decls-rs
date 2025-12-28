macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! entry {
    () => {
        deps!();
        pub (crate) mod entry { use super :: * ; pub (crate) mod prefix { use super :: * ; pub (crate) fn vis (p : & mut Parser < '_ >) { opt_visibility (p , false) ; } pub (crate) fn block (p : & mut Parser < '_ >) { expressions :: block_expr (p) ; } pub (crate) fn stmt (p : & mut Parser < '_ >) { expressions :: stmt (p , expressions :: Semicolon :: Forbidden) ; } pub (crate) fn pat (p : & mut Parser < '_ >) { patterns :: pattern_single (p) ; } pub (crate) fn pat_top (p : & mut Parser < '_ >) { patterns :: pattern (p) ; } pub (crate) fn ty (p : & mut Parser < '_ >) { types :: type_ (p) ; } pub (crate) fn expr (p : & mut Parser < '_ >) { expressions :: expr (p) ; } pub (crate) fn path (p : & mut Parser < '_ >) { paths :: type_path (p) ; } pub (crate) fn item (p : & mut Parser < '_ >) { items :: item_or_macro (p , true , true) ; } pub (crate) fn meta_item (p : & mut Parser < '_ >) { attributes :: meta (p) ; } } pub (crate) mod top { use super :: * ; pub (crate) fn source_file (p : & mut Parser < '_ >) { let m = p . start () ; p . eat (SHEBANG) ; p . eat (FRONTMATTER) ; items :: mod_contents (p , false) ; m . complete (p , SOURCE_FILE) ; } pub (crate) fn macro_stmts (p : & mut Parser < '_ >) { let m = p . start () ; while ! p . at (EOF) { expressions :: stmt (p , expressions :: Semicolon :: Optional) ; } m . complete (p , MACRO_STMTS) ; } pub (crate) fn macro_items (p : & mut Parser < '_ >) { let m = p . start () ; items :: mod_contents (p , false) ; m . complete (p , MACRO_ITEMS) ; } pub (crate) fn pattern (p : & mut Parser < '_ >) { let m = p . start () ; patterns :: pattern (p) ; if p . at (EOF) { m . abandon (p) ; return ; } while ! p . at (EOF) { p . bump_any () ; } m . complete (p , ERROR) ; } pub (crate) fn type_ (p : & mut Parser < '_ >) { let m = p . start () ; types :: type_ (p) ; if p . at (EOF) { m . abandon (p) ; return ; } while ! p . at (EOF) { p . bump_any () ; } m . complete (p , ERROR) ; } pub (crate) fn expr (p : & mut Parser < '_ >) { let m = p . start () ; expressions :: expr (p) ; if p . at (EOF) { m . abandon (p) ; return ; } while ! p . at (EOF) { p . bump_any () ; } m . complete (p , ERROR) ; } pub (crate) fn meta_item (p : & mut Parser < '_ >) { let m = p . start () ; attributes :: meta (p) ; if p . at (EOF) { m . abandon (p) ; return ; } while ! p . at (EOF) { p . bump_any () ; } m . complete (p , ERROR) ; } } }
    };
}

entry!()