// Generated macro for PLATFORM_C_FORWARD_DECLARATIONS (const)
macro_rules! Depcrate_arm_configPLATFORM_C_FORWARD_DECLARATIONS {
() => {
// Module: crate::arm::config
// Provides: {"PLATFORM_C_FORWARD_DECLARATIONS"}
// Dependencies: {}
pub const PLATFORM_C_FORWARD_DECLARATIONS : & str = r#"
#ifdef __aarch64__
std::ostream& operator<<(std::ostream& os, poly128_t value);
#endif

std::ostream& operator<<(std::ostream& os, float16_t value);
std::ostream& operator<<(std::ostream& os, uint8_t value);

// T1 is the `To` type, T2 is the `From` type
template<typename T1, typename T2> T1 cast(T2 x) {
  static_assert(sizeof(T1) == sizeof(T2), "sizeof T1 and T2 must be the same");
  T1 ret{};
  memcpy(&ret, &x, sizeof(T1));
  return ret;
}
"# ;
};
}
