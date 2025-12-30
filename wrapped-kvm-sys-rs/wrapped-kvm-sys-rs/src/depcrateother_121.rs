// Generated macro for other_121 (other)
macro_rules! Depcrateother_121 {
() => {
// Module: crate
// Provides: {"other_121"}
// Dependencies: {}
extern { fn kvm_get_api_version (fd : c_int) -> c_int ; fn kvm_create_vm (fd : c_int , flags : c_int) -> c_int ; fn kvm_check_extension (fd : c_int , extension : c_int) -> c_int ; fn kvm_get_vcpu_mmap_size (fd : c_int) -> c_int ; fn kvm_get_supported_cpuid (fd : c_int , cpuid : * mut Cpuid2) -> c_int ; fn kvm_create_vcpu (fd : c_int , vcpu_id : c_int) -> c_int ; fn kvm_set_user_memory_region (fd : c_int , region : * const UserspaceMemoryRegion) -> c_int ; fn kvm_run (fd : c_int) -> c_int ; fn kvm_get_regs (fd : c_int , regs : * mut Regs) -> c_int ; fn kvm_set_regs (fd : c_int , regs : * const Regs) -> c_int ; fn kvm_get_sregs (fd : c_int , sregs : * mut Sregs) -> c_int ; fn kvm_set_sregs (fd : c_int , sregs : * const Sregs) -> c_int ; fn kvm_set_cpuid2 (fd : c_int , cpuid : * const Cpuid2) -> c_int ; }
};
}
