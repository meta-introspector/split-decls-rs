// Generated macro for test (module)
macro_rules! Depcrate_ctxhashtest {
() => {
// Module: crate::ctxhash
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [derive (Clone , Copy , Debug)] struct Key { index : u32 , } struct Ctx { vals : & 'static [& 'static str] , } impl CtxEq < Key , Key > for Ctx { fn ctx_eq (& self , a : & Key , b : & Key) -> bool { self . vals [a . index as usize] . eq (self . vals [b . index as usize]) } } impl CtxHash < Key > for Ctx { fn ctx_hash < H : Hasher > (& self , state : & mut H , value : & Key) { self . vals [value . index as usize] . hash (state) ; } } # [test] fn test_basic () { let ctx = Ctx { vals : & ["a" , "b" , "a"] , } ; let k0 = Key { index : 0 } ; let k1 = Key { index : 1 } ; let k2 = Key { index : 2 } ; assert ! (ctx . ctx_eq (& k0 , & k2)) ; assert ! (! ctx . ctx_eq (& k0 , & k1)) ; assert ! (! ctx . ctx_eq (& k2 , & k1)) ; let mut map : CtxHashMap < Key , u64 > = CtxHashMap :: with_capacity (4) ; assert_eq ! (map . insert (k0 , 42 , & ctx) , None) ; assert_eq ! (map . insert (k2 , 84 , & ctx) , Some (42)) ; assert_eq ! (map . get (& k1 , & ctx) , None) ; assert_eq ! (* map . get (& k0 , & ctx) . unwrap () , 84) ; } }
};
}
