// Generated macro for POLY128_OSTREAM_DEF (const)
macro_rules! Depcrate_arm_configPOLY128_OSTREAM_DEF {
() => {
// Module: crate::arm::config
// Provides: {"POLY128_OSTREAM_DEF"}
// Dependencies: {}
pub const POLY128_OSTREAM_DEF : & str = r#"std::ostream& operator<<(std::ostream& os, poly128_t value) {
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
}"# ;
};
}
