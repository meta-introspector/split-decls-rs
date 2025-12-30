// Generated macro for tests (module)
macro_rules! Depcrate_internaltests {
() => {
// Module: crate::internal
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: error :: ErrorKind ; use crate :: bytes :: streaming :: { tag , take } ; use crate :: number :: streaming :: be_u16 ; use crate :: sequence :: terminated ; # [doc (hidden)] # [macro_export] macro_rules ! assert_size (($ t : ty , $ sz : expr) => (assert_eq ! ($ crate :: lib :: std :: mem :: size_of ::<$ t > () , $ sz) ;) ;) ; # [test] # [cfg (target_pointer_width = "64")] fn size_test () { assert_size ! (IResult <& [u8] , & [u8] , (& [u8] , u32) >, 40) ; assert_size ! (Needed , 8) ; assert_size ! (Err < u32 >, 16) ; assert_size ! (ErrorKind , 1) ; } # [test] fn err_map_test () { let e = Err :: Error (1) ; assert_eq ! (e . map (| v | v + 1) , Err :: Error (2)) ; } # [test] fn native_tuple_test () { fn tuple_3 (i : & [u8]) -> IResult < & [u8] , (u16 , & [u8]) > { terminated ((be_u16 , take (3u8)) , tag ("fg")) . parse (i) } assert_eq ! (tuple_3 (& b"abcdefgh" [..]) , Ok ((& b"h" [..] , (0x6162u16 , & b"cde" [..])))) ; assert_eq ! (tuple_3 (& b"abcd" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (tuple_3 (& b"abcde" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (tuple_3 (& b"abcdejk" [..]) , Err (Err :: Error (error_position ! (& b"jk" [..] , ErrorKind :: Tag)))) ; } }
};
}
