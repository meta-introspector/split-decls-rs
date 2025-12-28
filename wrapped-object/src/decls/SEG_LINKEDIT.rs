macro_rules! SEG_LINKEDIT {
    () => {
        # [doc = " the segment containing all structs created and maintained by the link editor.  Created with -seglinkedit option to ld(1) for MH_EXECUTE and FVMLIB file types only"] pub const SEG_LINKEDIT : & str = "__LINKEDIT" ;
    };
}

SEG_LINKEDIT!()