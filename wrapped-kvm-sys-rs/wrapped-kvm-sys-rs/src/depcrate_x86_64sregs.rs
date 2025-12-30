// Generated macro for Sregs (struct)
macro_rules! Depcrate_x86_64Sregs {
() => {
// Module: crate::x86_64
// Provides: {"Sregs"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Debug)] pub struct Sregs { pub cs : Segment , pub ds : Segment , pub es : Segment , pub fs : Segment , pub gs : Segment , pub ss : Segment , pub tr : Segment , pub ldt : Segment , pub gdt : Dtable , pub idt : Dtable , pub cr0 : u64 , pub cr2 : u64 , pub cr3 : u64 , pub cr4 : u64 , pub cr8 : u64 , pub efer : u64 , pub apic_base : u64 , pub interrupt_bitmap : [u64 ; 4usize] , }
};
}
