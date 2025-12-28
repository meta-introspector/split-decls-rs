macro_rules! deps {
    () => {
        StoreBytes!();
        Store!();
        AndNot!();
        BSwap!();
        BitOps0!();
    };
}

macro_rules! def_vec {
    () => {
        deps!();
        macro_rules ! def_vec { ($ vec : ident , $ word : ident) => { zerocopy :: cryptocorrosion_derive_traits ! { # [repr (transparent)] # [allow (non_camel_case_types)] # [derive (Copy , Clone)] pub struct $ vec < S3 , S4 , NI > { x : __m128i , s3 : PhantomData < S3 >, s4 : PhantomData < S4 >, ni : PhantomData < NI >, } } impl < S3 , S4 , NI > Store < vec128_storage > for $ vec < S3 , S4 , NI > { # [inline (always)] unsafe fn unpack (x : vec128_storage) -> Self { Self :: new (x . sse2) } } impl < S3 , S4 , NI > From <$ vec < S3 , S4 , NI >> for vec128_storage { # [inline (always)] fn from (x : $ vec < S3 , S4 , NI >) -> Self { vec128_storage { sse2 : x . x } } } impl < S3 , S4 , NI > $ vec < S3 , S4 , NI > { # [inline (always)] fn new (x : __m128i) -> Self { $ vec { x , s3 : PhantomData , s4 : PhantomData , ni : PhantomData , } } } impl < S3 , S4 , NI > StoreBytes for $ vec < S3 , S4 , NI > where Self : BSwap , { # [inline (always)] unsafe fn unsafe_read_le (input : & [u8]) -> Self { assert_eq ! (input . len () , 16) ; Self :: new (_mm_loadu_si128 (input . as_ptr () as * const _)) } # [inline (always)] unsafe fn unsafe_read_be (input : & [u8]) -> Self { assert_eq ! (input . len () , 16) ; Self :: new (_mm_loadu_si128 (input . as_ptr () as * const _)) . bswap () } # [inline (always)] fn write_le (self , out : & mut [u8]) { assert_eq ! (out . len () , 16) ; unsafe { _mm_storeu_si128 (out . as_mut_ptr () as * mut _ , self . x) } } # [inline (always)] fn write_be (self , out : & mut [u8]) { assert_eq ! (out . len () , 16) ; let x = self . bswap () . x ; unsafe { _mm_storeu_si128 (out . as_mut_ptr () as * mut _ , x) ; } } } impl < S3 , S4 , NI > Default for $ vec < S3 , S4 , NI > { # [inline (always)] fn default () -> Self { Self :: new (unsafe { _mm_setzero_si128 () }) } } impl < S3 , S4 , NI > Not for $ vec < S3 , S4 , NI > { type Output = Self ; # [inline (always)] fn not (self) -> Self :: Output { unsafe { let ff = _mm_set1_epi64x (- 1i64) ; self ^ Self :: new (ff) } } } impl < S3 : Copy , S4 : Copy , NI : Copy > BitOps0 for $ vec < S3 , S4 , NI > { } impl_binop ! ($ vec , BitAnd , bitand , _mm_and_si128) ; impl_binop ! ($ vec , BitOr , bitor , _mm_or_si128) ; impl_binop ! ($ vec , BitXor , bitxor , _mm_xor_si128) ; impl_binop_assign ! ($ vec , BitAndAssign , bitand_assign , bitand) ; impl_binop_assign ! ($ vec , BitOrAssign , bitor_assign , bitor) ; impl_binop_assign ! ($ vec , BitXorAssign , bitxor_assign , bitxor) ; impl < S3 : Copy , S4 : Copy , NI : Copy > AndNot for $ vec < S3 , S4 , NI > { type Output = Self ; # [inline (always)] fn andnot (self , rhs : Self) -> Self { Self :: new (unsafe { _mm_andnot_si128 (self . x , rhs . x) }) } } } ; }
    };
}

def_vec!();