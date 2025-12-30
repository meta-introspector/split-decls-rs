// Generated macro for PLATFORM_C_FORWARD_DECLARATIONS (const)
macro_rules! Depcrate_x86_configPLATFORM_C_FORWARD_DECLARATIONS {
() => {
// Module: crate::x86::config
// Provides: {"PLATFORM_C_FORWARD_DECLARATIONS"}
// Dependencies: {}
pub const PLATFORM_C_FORWARD_DECLARATIONS : & str = r#"
#ifndef X86_DECLARATIONS
#define X86_DECLARATIONS
    typedef _Float16 float16_t;
    typedef float float32_t;
    typedef double float64_t;
    
    #define __int64 long long
    #define __int32 int

    std::ostream& operator<<(std::ostream& os, _Float16 value);
    std::ostream& operator<<(std::ostream& os, __m128i value);
    std::ostream& operator<<(std::ostream& os, __m256i value);
    std::ostream& operator<<(std::ostream& os, __m512i value);
    std::ostream& operator<<(std::ostream& os, __mmask8 value);
    
    #define _mm512_extract_intrinsic_test_epi8(m, lane) \
        _mm_extract_epi8(_mm512_extracti64x2_epi64((m), (lane) / 16), (lane) % 16)
    
    #define _mm512_extract_intrinsic_test_epi16(m, lane) \
        _mm_extract_epi16(_mm512_extracti64x2_epi64((m), (lane) / 8), (lane) % 8)
    
    #define _mm512_extract_intrinsic_test_epi32(m, lane) \
        _mm_extract_epi32(_mm512_extracti64x2_epi64((m), (lane) / 4), (lane) % 4)
    
    #define _mm512_extract_intrinsic_test_epi64(m, lane) \
        _mm_extract_epi64(_mm512_extracti64x2_epi64((m), (lane) / 2), (lane) % 2)
        
    // Load f16 (__m128h) and cast to integer (__m128i)
    #define _mm_loadu_ph_to___m128i(mem_addr) _mm_castph_si128(_mm_loadu_ph(mem_addr))
    #define _mm256_loadu_ph_to___m256i(mem_addr) _mm256_castph_si256(_mm256_loadu_ph(mem_addr))
    #define _mm512_loadu_ph_to___m512i(mem_addr) _mm512_castph_si512(_mm512_loadu_ph(mem_addr))
    
    // Load f32 (__m128) and cast to f16 (__m128h)
    #define _mm_loadu_ps_to___m128h(mem_addr) _mm_castps_ph(_mm_loadu_ps(mem_addr))
    #define _mm256_loadu_ps_to___m256h(mem_addr) _mm256_castps_ph(_mm256_loadu_ps(mem_addr))
    #define _mm512_loadu_ps_to___m512h(mem_addr) _mm512_castps_ph(_mm512_loadu_ps(mem_addr))
    
    // Load integer types and cast to double (__m128d, __m256d, __m512d)
    #define _mm_loadu_epi16_to___m128d(mem_addr) _mm_castsi128_pd(_mm_loadu_si128((__m128i const*)(mem_addr)))
    #define _mm256_loadu_epi16_to___m256d(mem_addr) _mm256_castsi256_pd(_mm256_loadu_si256((__m256i const*)(mem_addr)))
    #define _mm512_loadu_epi16_to___m512d(mem_addr) _mm512_castsi512_pd(_mm512_loadu_si512((__m512i const*)(mem_addr)))
    
    #define _mm_loadu_epi32_to___m128d(mem_addr) _mm_castsi128_pd(_mm_loadu_si128((__m128i const*)(mem_addr)))
    #define _mm256_loadu_epi32_to___m256d(mem_addr) _mm256_castsi256_pd(_mm256_loadu_si256((__m256i const*)(mem_addr)))
    #define _mm512_loadu_epi32_to___m512d(mem_addr) _mm512_castsi512_pd(_mm512_loadu_si512((__m512i const*)(mem_addr)))
    
    #define _mm_loadu_epi64_to___m128d(mem_addr) _mm_castsi128_pd(_mm_loadu_si128((__m128i const*)(mem_addr)))
    #define _mm256_loadu_epi64_to___m256d(mem_addr) _mm256_castsi256_pd(_mm256_loadu_si256((__m256i const*)(mem_addr)))
    #define _mm512_loadu_epi64_to___m512d(mem_addr) _mm512_castsi512_pd(_mm512_loadu_si512((__m512i const*)(mem_addr)))
    
    // Load integer types and cast to float (__m128, __m256, __m512)
    #define _mm_loadu_epi16_to___m128(mem_addr) _mm_castsi128_ps(_mm_loadu_si128((__m128i const*)(mem_addr)))
    #define _mm256_loadu_epi16_to___m256(mem_addr) _mm256_castsi256_ps(_mm256_loadu_si256((__m256i const*)(mem_addr)))
    #define _mm512_loadu_epi16_to___m512(mem_addr) _mm512_castsi512_ps(_mm512_loadu_si512((__m512i const*)(mem_addr)))
    
    #define _mm_loadu_epi32_to___m128(mem_addr) _mm_castsi128_ps(_mm_loadu_si128((__m128i const*)(mem_addr)))
    #define _mm256_loadu_epi32_to___m256(mem_addr) _mm256_castsi256_ps(_mm256_loadu_si256((__m256i const*)(mem_addr)))
    #define _mm512_loadu_epi32_to___m512(mem_addr) _mm512_castsi512_ps(_mm512_loadu_si512((__m512i const*)(mem_addr)))
    
    #define _mm_loadu_epi64_to___m128(mem_addr) _mm_castsi128_ps(_mm_loadu_si128((__m128i const*)(mem_addr)))
    #define _mm256_loadu_epi64_to___m256(mem_addr) _mm256_castsi256_ps(_mm256_loadu_si256((__m256i const*)(mem_addr)))
    #define _mm512_loadu_epi64_to___m512(mem_addr) _mm512_castsi512_ps(_mm512_loadu_si512((__m512i const*)(mem_addr)))
    
    // T1 is the `To` type, T2 is the `From` type
    template<typename T1, typename T2> T1 cast(T2 x) {
      if constexpr ((std::is_integral_v<T1> && std::is_integral_v<T2>) || (std::is_floating_point_v<T1> && std::is_floating_point_v<T2>)) {
          return x;
      } else if constexpr (sizeof(T1) <= sizeof(T2)) {
        T1 ret{};
        std::memcpy(&ret, &x, sizeof(T1));
        return ret;
      } else {
        static_assert(sizeof(T1) == sizeof(T2) || std::is_convertible_v<T2, T1>,
                              "T2 must either be convertible to T1, or have the same size as T1!");
        return T1{};
      }
    }
#endif
"# ;
};
}
