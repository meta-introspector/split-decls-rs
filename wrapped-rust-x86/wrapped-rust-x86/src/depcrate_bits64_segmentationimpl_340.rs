// Generated macro for impl_340 (impl)
macro_rules! Depcrate_bits64_segmentationimpl_340 {
() => {
// Module: crate::bits64::segmentation
// Provides: {"impl_340"}
// Dependencies: {}
impl BuildDescriptor < Descriptor64 > for DescriptorBuilder { fn finish (& self) -> Descriptor64 { let mut desc : Descriptor64 = Default :: default () ; desc . apply_builder_settings (self) ; let typ = match self . typ { Some (DescriptorType :: System64 (typ)) => { assert ! (! self . l) ; if typ == SystemDescriptorTypes64 :: LDT || typ == SystemDescriptorTypes64 :: TssAvailable || typ == SystemDescriptorTypes64 :: TssBusy { assert ! (! self . db) ; } if typ == SystemDescriptorTypes64 :: InterruptGate { desc . set_ist (self . ist) ; } typ as u8 } Some (DescriptorType :: System32 (_typ)) => { panic ! ("Can't build a 64-bit version of this type.") } Some (DescriptorType :: Data (_typ)) => { panic ! ("Can't build a 64-bit version of this type.") } Some (DescriptorType :: Code (_typ)) => { panic ! ("Can't build a 64-bit version of this type.") } None => unreachable ! ("Type not set, this is a library bug in x86.") , } ; desc . desc32 . set_type (typ) ; desc } }
};
}
