macro_rules! deps {
    () => {
        Id!();
        Error!();
        Cursor!();
    };
}

macro_rules! write_cursor {
    () => {
        deps!();
        fn write_cursor < 'repo > (cursor : & mut Cursor < '_ , 'repo >) -> Result < Id < 'repo > , write :: Error > { cursor . inner . write (| tree | -> Result < ObjectId , write :: Error > { for entry in & tree . entries { gix_validate :: path :: component (entry . filename . as_ref () , entry . mode . is_link () . then_some (gix_validate :: path :: component :: Mode :: Symlink) , cursor . validate ,) . map_err (| err | write :: Error :: InvalidFilename { filename : entry . filename . clone () , kind : entry . mode . into () , id : entry . oid , source : err , }) ? ; if ! entry . mode . is_commit () && ! cursor . repo . has_object (entry . oid) { return Err (write :: Error :: MissingObject { filename : entry . filename . clone () , kind : entry . mode . into () , id : entry . oid , }) ; } } Ok (cursor . repo . write_object (tree) ? . detach ()) }) . map (| id | id . attach (cursor . repo)) }
    };
}

write_cursor!();