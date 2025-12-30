// Generated macro for ImageEntryPoint (type)
macro_rules! Depcrate_baseImageEntryPoint {
() => {
// Module: crate::base
// Provides: {"ImageEntryPoint"}
// Dependencies: {}
# [doc = " Application Entry Point"] # [doc = ""] # [doc = " This type defines the entry-point of UEFI applications. It is ABI and cannot be changed."] # [doc = " Whenever you load UEFI images, the entry-point is called with this signature."] # [doc = ""] # [doc = " In most cases the UEFI image (or application) is unloaded when control returns from the entry"] # [doc = " point. In case of UEFI drivers, they can request to stay loaded until an explicit unload."] # [doc = ""] # [doc = " The system table is provided as mutable pointer. This is, because there is no guarantee that"] # [doc = " timer interrupts do not modify the table. Furthermore, exiting boot services causes several"] # [doc = " modifications on that table. And lastly, the system table lives longer than the function"] # [doc = " invocation, if invoked as an UEFI driver."] # [doc = " In most cases it is perfectly fine to cast the pointer to a real rust reference. However, this"] # [doc = " should be an explicit decision by the caller."] pub type ImageEntryPoint = eficall ! { fn (Handle , * mut crate :: system :: SystemTable) -> Status } ;
};
}
