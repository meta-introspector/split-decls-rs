macro_rules! name {
    () => {
        # [doc = " Return the original `name` if it is valid, or the respective error indicating what was wrong with it."] pub fn name (name : & BStr) -> Result < & BStr , name :: Error > { if name . is_empty () { return Err (name :: Error :: Empty) ; } match name . find (b"..") { Some (pos) => { let & b = name . get (pos + 2) . ok_or (name :: Error :: ParentComponent) ? ; if b == b'/' || b == b'\\' { Err (name :: Error :: ParentComponent) } else { Ok (name) } } None => Ok (name) , } }
    };
}

name!()