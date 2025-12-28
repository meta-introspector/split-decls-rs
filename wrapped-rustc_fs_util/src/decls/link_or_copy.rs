macro_rules! deps {
    () => {
        LinkOrCopy!();
    };
}

macro_rules! link_or_copy {
    () => {
        deps!();
        # [doc = " Copies `p` into `q`, preferring to use hard-linking if possible."] # [doc = " The result indicates which of the two operations has been performed."] pub fn link_or_copy < P : AsRef < Path > , Q : AsRef < Path > > (p : P , q : Q) -> io :: Result < LinkOrCopy > { let p = p . as_ref () ; let q = q . as_ref () ; let err = match fs :: hard_link (p , q) { Ok (()) => return Ok (LinkOrCopy :: Link) , Err (err) => err , } ; if err . kind () == io :: ErrorKind :: AlreadyExists { fs :: remove_file (q) ? ; if fs :: hard_link (p , q) . is_ok () { return Ok (LinkOrCopy :: Link) ; } } fs :: copy (p , q) . map (| _ | LinkOrCopy :: Copy) }
    };
}

link_or_copy!();