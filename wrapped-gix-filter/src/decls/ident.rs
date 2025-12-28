macro_rules! ident {
    () => {
        # [doc = " a filter to replace `$Id$` with a git-hash of the buffer."] pub mod ident ;
    };
}

ident!();