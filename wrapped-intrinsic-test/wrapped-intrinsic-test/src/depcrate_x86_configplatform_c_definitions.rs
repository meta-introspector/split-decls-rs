// Generated macro for PLATFORM_C_DEFINITIONS (const)
macro_rules! Depcrate_x86_configPLATFORM_C_DEFINITIONS {
() => {
// Module: crate::x86::config
// Provides: {"PLATFORM_C_DEFINITIONS"}
// Dependencies: {}
pub const PLATFORM_C_DEFINITIONS : & str = r#"

std::ostream& operator<<(std::ostream& os, _Float16 value) {
    os << static_cast<float>(value);
    return os;
}

std::ostream& operator<<(std::ostream& os, __m128i value) {
    void* temp = malloc(sizeof(__m128i));
    _mm_storeu_si128((__m128i*)temp, value);
    std::stringstream ss;
    
    ss << "0x";
    for(int i = 0; i < 16; i++) {
        ss << std::setfill('0') << std::setw(2) << std::hex << ((char*)temp)[i];
    }
    os << ss.str();
    return os;
}

std::ostream& operator<<(std::ostream& os, __m256i value) {
    void* temp = malloc(sizeof(__m256i));
    _mm256_storeu_si256((__m256i*)temp, value);
    std::stringstream ss;
    
    ss << "0x";
    for(int i = 0; i < 32; i++) {
        ss << std::setfill('0') << std::setw(2) << std::hex << ((char*)temp)[i];
    }
    os << ss.str();
    return os;
}

std::ostream& operator<<(std::ostream& os, __m512i value) {
    void* temp = malloc(sizeof(__m512i));
    _mm512_storeu_si512((__m512i*)temp, value);
    std::stringstream ss;
    
    ss << "0x";
    for(int i = 0; i < 64; i++) {
        ss << std::setfill('0') << std::setw(2) << std::hex << ((char*)temp)[i];
    }
    os << ss.str();
    return os;
}

std::ostream& operator<<(std::ostream& os, __mmask8 value) {
    os << static_cast<int>(value);
    return os;
}
"# ;
};
}
