// Generated macro for other_925 (other)
macro_rules! Depcrate_xml_nodeother_925 {
() => {
// Module: crate::xml_node
// Provides: {"other_925"}
// Dependencies: {}
unsafe extern "C" { pub fn CFXMLNodeGetTypeID () -> CFTypeID ; pub fn CFXMLNodeCreate (alloc : CFAllocatorRef , xmlType : CFXMLNodeTypeCode , dataString : CFStringRef , additionalInfoPtr : * const c_void , version : CFIndex ,) -> CFXMLNodeRef ; pub fn CFXMLNodeCreateCopy (alloc : CFAllocatorRef , origNode : CFXMLNodeRef) -> CFXMLNodeRef ; pub fn CFXMLNodeGetTypeCode (node : CFXMLNodeRef) -> CFXMLNodeTypeCode ; pub fn CFXMLNodeGetString (node : CFXMLNodeRef) -> CFStringRef ; pub fn CFXMLNodeGetInfoPtr (node : CFXMLNodeRef) -> * const c_void ; pub fn CFXMLNodeGetVersion (node : CFXMLNodeRef) -> CFIndex ; pub fn CFXMLTreeCreateWithNode (alloc : CFAllocatorRef , node : CFXMLNodeRef) -> CFXMLTreeRef ; pub fn CFXMLTreeGetNode (xmlTree : CFXMLTreeRef) -> CFXMLNodeRef ; }
};
}
