macro_rules! LARGE_POW5 {
    () => {
        # [doc = " Pre-computed large power-of-5 for 64-bit limbs."] # [cfg (all (target_pointer_width = "64" , not (target_arch = "sparc")))] pub const LARGE_POW5 : [u64 ; 5] = [1414648277510068013 , 9180637584431281687 , 4539964771860779200 , 10482974169319127550 , 198276706040285095 ,] ;
    };
}

LARGE_POW5!();