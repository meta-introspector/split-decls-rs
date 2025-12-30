// Generated macro for tests (module)
macro_rules! Depcrate_simpletests {
() => {
// Module: crate::simple
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: io ; use csv ; use super :: RandomAccessSimple ; struct Indexed < 'a > { csv : csv :: Reader < io :: Cursor < & 'a str > > , idx : RandomAccessSimple < io :: Cursor < Vec < u8 > > > , } impl < 'a > Indexed < 'a > { fn new (headers : bool , csv_data : & 'a str) -> Indexed < 'a > { let mut rdr = csv :: ReaderBuilder :: new () . has_headers (headers) . from_reader (io :: Cursor :: new (csv_data)) ; let mut idxbuf = io :: Cursor :: new (vec ! []) ; RandomAccessSimple :: create (& mut rdr , & mut idxbuf) . unwrap () ; Indexed { csv : rdr , idx : RandomAccessSimple :: open (idxbuf) . unwrap () , } } fn read_at (& mut self , record : u64) -> csv :: StringRecord { let pos = self . idx . get (record) . unwrap () ; self . csv . seek (pos) . unwrap () ; self . csv . records () . next () . unwrap () . unwrap () } } # [test] fn headers_empty () { let idx = Indexed :: new (true , "") ; assert_eq ! (idx . idx . len () , 0) ; } # [test] fn headers_one_field () { let mut idx = Indexed :: new (true , "h1\na\nb\nc\n") ; assert_eq ! (idx . idx . len () , 4) ; assert_eq ! (idx . read_at (0) , vec ! ["h1"]) ; assert_eq ! (idx . read_at (1) , vec ! ["a"]) ; assert_eq ! (idx . read_at (2) , vec ! ["b"]) ; assert_eq ! (idx . read_at (3) , vec ! ["c"]) ; } # [test] fn headers_many_fields () { let mut idx = Indexed :: new (true , "\
h1,h2,h3
a,b,c
d,e,f
g,h,i
" ,) ; assert_eq ! (idx . idx . len () , 4) ; assert_eq ! (idx . read_at (0) , vec ! ["h1" , "h2" , "h3"]) ; assert_eq ! (idx . read_at (1) , vec ! ["a" , "b" , "c"]) ; assert_eq ! (idx . read_at (2) , vec ! ["d" , "e" , "f"]) ; assert_eq ! (idx . read_at (3) , vec ! ["g" , "h" , "i"]) ; } # [test] fn no_headers_one_field () { let mut idx = Indexed :: new (false , "h1\na\nb\nc\n") ; assert_eq ! (idx . idx . len () , 4) ; assert_eq ! (idx . read_at (0) , vec ! ["h1"]) ; assert_eq ! (idx . read_at (1) , vec ! ["a"]) ; assert_eq ! (idx . read_at (2) , vec ! ["b"]) ; assert_eq ! (idx . read_at (3) , vec ! ["c"]) ; } # [test] fn no_headers_many_fields () { let mut idx = Indexed :: new (false , "\
h1,h2,h3
a,b,c
d,e,f
g,h,i
" ,) ; assert_eq ! (idx . idx . len () , 4) ; assert_eq ! (idx . read_at (0) , vec ! ["h1" , "h2" , "h3"]) ; assert_eq ! (idx . read_at (1) , vec ! ["a" , "b" , "c"]) ; assert_eq ! (idx . read_at (2) , vec ! ["d" , "e" , "f"]) ; assert_eq ! (idx . read_at (3) , vec ! ["g" , "h" , "i"]) ; } # [test] fn headers_one_field_newlines () { let mut idx = Indexed :: new (true , "




h1

a


b






c






" ,) ; assert_eq ! (idx . idx . len () , 4) ; assert_eq ! (idx . read_at (0) , vec ! ["h1"]) ; assert_eq ! (idx . read_at (1) , vec ! ["a"]) ; assert_eq ! (idx . read_at (2) , vec ! ["b"]) ; assert_eq ! (idx . read_at (3) , vec ! ["c"]) ; } }
};
}
