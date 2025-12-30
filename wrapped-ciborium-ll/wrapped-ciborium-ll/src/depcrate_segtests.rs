// Generated macro for tests (module)
macro_rules! Depcrate_segtests {
() => {
// Module: crate::seg
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn segments () { fn t (data : & [u8] , len : usize) { let mut dec = Decoder :: from (data) ; let mut segs = Segments :: < _ , Bytes > :: new (& mut dec , | header | match header { Header :: Bytes (len) => Ok (len) , _ => Err (()) , }) ; while let Some (mut seg) = segs . pull () . unwrap () { let mut b = [0 ; 1] ; assert_eq ! (Some (& b"0" [..]) , seg . pull (& mut b) . unwrap ()) ; } assert_eq ! (len , dec . offset ()) ; } t (b"\x410\x00" , 2) ; t (b"\x5f\x410\xff\x00" , 4) ; } }
};
}
