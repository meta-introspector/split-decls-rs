macro_rules! deps {
    () => {
        State!();
        PpAnn!();
        Comments!();
        Printer!();
    };
}

macro_rules! print_crate {
    () => {
        deps!();
        # [doc = " Requires you to pass an input filename and reader so that"] # [doc = " it can scan the input text for comments to copy forward."] pub fn print_crate < 'a > (sm : & 'a SourceMap , krate : & ast :: Crate , filename : FileName , input : String , ann : & 'a dyn PpAnn , is_expanded : bool , edition : Edition , g : & AttrIdGenerator ,) -> String { let mut s = State { s : pp :: Printer :: new () , comments : Some (Comments :: new (sm , filename , input)) , ann , is_sdylib_interface : false , } ; print_crate_inner (& mut s , krate , is_expanded , edition , g) ; s . s . eof () }
    };
}

print_crate!();