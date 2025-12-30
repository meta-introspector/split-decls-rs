// Generated macro for PLATFORM_C_DEFINITIONS (const)
macro_rules! Depcrate_arm_configPLATFORM_C_DEFINITIONS {
() => {
// Module: crate::arm::config
// Provides: {"PLATFORM_C_DEFINITIONS"}
// Dependencies: {}
pub const PLATFORM_C_DEFINITIONS : & str = r#"
#ifdef __aarch64__
std::ostream& operator<<(std::ostream& os, poly128_t value) {
    std::stringstream temp;
    do {
      int n = value % 10;
      value /= 10;
      temp << n;
    } while (value != 0);
    std::string tempstr(temp.str());
    std::string res(tempstr.rbegin(), tempstr.rend());
    os << res;
    return os;
}

#endif

std::ostream& operator<<(std::ostream& os, float16_t value) {
    os << static_cast<float>(value);
    return os;
}

std::ostream& operator<<(std::ostream& os, uint8_t value) {
    os << (unsigned int) value;
    return os;
}
"# ;
};
}
