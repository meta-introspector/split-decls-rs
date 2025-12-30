// Generated macro for PLATFORM_RUST_DEFINITIONS (const)
macro_rules! Depcrate_x86_configPLATFORM_RUST_DEFINITIONS {
() => {
// Module: crate::x86::config
// Provides: {"PLATFORM_RUST_DEFINITIONS"}
// Dependencies: {}
pub const PLATFORM_RUST_DEFINITIONS : & str = r#"
use core_arch::arch::x86_64::*;

#[inline]
unsafe fn _mm_loadu_ph_to___m128i(mem_addr: *const f16) -> __m128i {
    _mm_castph_si128(_mm_loadu_ph(mem_addr))
}

#[inline]
unsafe fn _mm256_loadu_ph_to___m256i(mem_addr: *const f16) -> __m256i {
    _mm256_castph_si256(_mm256_loadu_ph(mem_addr))
}

#[inline]
unsafe fn _mm512_loadu_ph_to___mm512i(mem_addr: *const f16) -> __m512i {
    _mm512_castph_si512(_mm512_loadu_ph(mem_addr))
}


#[inline]
unsafe fn _mm_loadu_ps_to___m128h(mem_addr: *const f32) -> __m128h {
    _mm_castps_ph(_mm_loadu_ps(mem_addr))
}

#[inline]
unsafe fn _mm256_loadu_ps_to___m256h(mem_addr: *const f32) -> __m256h {
    _mm256_castps_ph(_mm256_loadu_ps(mem_addr))
}

#[inline]
unsafe fn _mm512_loadu_ps_to___m512h(mem_addr: *const f32) -> __m512h {
    _mm512_castps_ph(_mm512_loadu_ps(mem_addr))
}

#[inline]
unsafe fn _mm_loadu_epi16_to___m128d(mem_addr: *const i16) -> __m128d {
    _mm_castsi128_pd(_mm_loadu_epi16(mem_addr))
}

#[inline]
unsafe fn _mm256_loadu_epi16_to___m256d(mem_addr: *const i16) -> __m256d {
    _mm256_castsi256_pd(_mm256_loadu_epi16(mem_addr))
}

#[inline]
unsafe fn _mm512_loadu_epi16_to___m512d(mem_addr: *const i16) -> __m512d {
    _mm512_castsi512_pd(_mm512_loadu_epi16(mem_addr))
}

#[inline]
unsafe fn _mm_loadu_epi32_to___m128d(mem_addr: *const i32) -> __m128d {
    _mm_castsi128_pd(_mm_loadu_epi32(mem_addr))
}

#[inline]
unsafe fn _mm256_loadu_epi32_to___m256d(mem_addr: *const i32) -> __m256d {
    _mm256_castsi256_pd(_mm256_loadu_epi32(mem_addr))
}

#[inline]
unsafe fn _mm512_loadu_epi32_to___m512d(mem_addr: *const i32) -> __m512d {
    _mm512_castsi512_pd(_mm512_loadu_epi32(mem_addr))
}

#[inline]
unsafe fn _mm_loadu_epi64_to___m128d(mem_addr: *const i64) -> __m128d {
    _mm_castsi128_pd(_mm_loadu_epi64(mem_addr))
}

#[inline]
unsafe fn _mm256_loadu_epi64_to___m256d(mem_addr: *const i64) -> __m256d {
    _mm256_castsi256_pd(_mm256_loadu_epi64(mem_addr))
}

#[inline]
unsafe fn _mm512_loadu_epi64_to___m512d(mem_addr: *const i64) -> __m512d {
    _mm512_castsi512_pd(_mm512_loadu_epi64(mem_addr))
}

// === 
#[inline]
unsafe fn _mm_loadu_epi16_to___m128(mem_addr: *const i16) -> __m128 {
    _mm_castsi128_ps(_mm_loadu_epi16(mem_addr))
}

#[inline]
unsafe fn _mm256_loadu_epi16_to___m256(mem_addr: *const i16) -> __m256 {
    _mm256_castsi256_ps(_mm256_loadu_epi16(mem_addr))
}

#[inline]
unsafe fn _mm512_loadu_epi16_to___m512(mem_addr: *const i16) -> __m512 {
    _mm512_castsi512_ps(_mm512_loadu_epi16(mem_addr))
}

#[inline]
unsafe fn _mm_loadu_epi32_to___m128(mem_addr: *const i32) -> __m128 {
    _mm_castsi128_ps(_mm_loadu_epi32(mem_addr))
}

#[inline]
unsafe fn _mm256_loadu_epi32_to___m256(mem_addr: *const i32) -> __m256 {
    _mm256_castsi256_ps(_mm256_loadu_epi32(mem_addr))
}

#[inline]
unsafe fn _mm512_loadu_epi32_to___m512(mem_addr: *const i32) -> __m512 {
    _mm512_castsi512_ps(_mm512_loadu_epi32(mem_addr))
}

#[inline]
unsafe fn _mm_loadu_epi64_to___m128(mem_addr: *const i64) -> __m128 {
    _mm_castsi128_ps(_mm_loadu_epi64(mem_addr))
}

#[inline]
unsafe fn _mm256_loadu_epi64_to___m256(mem_addr: *const i64) -> __m256 {
    _mm256_castsi256_ps(_mm256_loadu_epi64(mem_addr))
}

#[inline]
unsafe fn _mm512_loadu_epi64_to___m512(mem_addr: *const i64) -> __m512 {
    _mm512_castsi512_ps(_mm512_loadu_epi64(mem_addr))
}

#[inline]
fn debug_simd_finish<T: core::fmt::Debug, const N: usize>(
    formatter: &mut core::fmt::Formatter<'_>,
    type_name: &str,
    array: &[T; N],
) -> core::fmt::Result {
    core::fmt::Formatter::debug_tuple_fields_finish(
        formatter,
        type_name,
        &core::array::from_fn::<&dyn core::fmt::Debug, N, _>(|i| &array[i]),
    )
}

trait DebugAs<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

impl<T: core::fmt::Display> DebugAs<T> for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self}")
    }
}

macro_rules! impl_debug_as {
    ($simd:ty, $name:expr, $bits:expr, [$($type:ty),+]) => {
        $(
            impl DebugAs<$type> for $simd {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    const ELEMENT_BITS: usize = core::mem::size_of::<$type>() * 8;
                    const NUM_ELEMENTS: usize = $bits / ELEMENT_BITS;
                    let array = unsafe { core::mem::transmute::<_, [$type; NUM_ELEMENTS]>(*self) };
                    debug_simd_finish(f, $name, &array)
                }
            }
        )+
    };
}

impl_debug_as!(__m128i, "__m128i", 128, [u8, i8, u16, i16, u32, i32, u64, i64, f16]);
impl_debug_as!(__m256i, "__m256i", 256, [u8, i8, u16, i16, u32, i32, u64, i64]);
impl_debug_as!(__m512i, "__m512i", 512, [u8, i8, u16, i16, u32, i32, u64, i64]);
impl_debug_as!(__m128h, "__m128h", 128, [f32]);
impl_debug_as!(__m256h, "__m256h", 256, [f32]);
impl_debug_as!(__m512h, "__m512h", 512, [f32]);

fn debug_as<V, T>(x: V) -> impl core::fmt::Debug 
where V: DebugAs<T>
{
    struct DebugWrapper<V, T>(V, core::marker::PhantomData<T>);
    impl<V: DebugAs<T>, T> core::fmt::Debug for DebugWrapper<V, T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
    DebugWrapper(x, core::marker::PhantomData)
}

"# ;
};
}
