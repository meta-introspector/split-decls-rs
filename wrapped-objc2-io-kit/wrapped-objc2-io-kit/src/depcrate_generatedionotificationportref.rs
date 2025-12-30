// Generated macro for IONotificationPortRef (type)
macro_rules! Depcrate_generatedIONotificationPortRef {
() => {
// Module: crate::generated
// Provides: {"IONotificationPortRef"}
// Dependencies: {}
# [doc = " IOKitLib"] # [doc = " IOKitLib implements non-kernel task access to common IOKit object types - IORegistryEntry, IOService, IOIterator etc. These functions are generic - families may provide API that is more specific."] # [doc = " <br>"] # [doc = " IOKitLib represents IOKit objects outside the kernel with the types io_object_t, io_registry_entry_t, io_service_t,"] # [doc = " &"] # [doc = " io_connect_t. Function names usually begin with the type of object they are compatible with - eg. IOObjectRelease can be used with any io_object_t. Inside the kernel, the c++ class hierarchy allows the subclasses of each object type to receive the same requests from user level clients, for example in the kernel, IOService is a subclass of IORegistryEntry, which means any of the IORegistryEntryXXX functions in IOKitLib may be used with io_service_t's as well as io_registry_t's. There are functions available to introspect the class of the kernel object which any io_object_t et al. represents."] # [doc = " IOKit objects returned by all functions should be released with IOObjectRelease."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/ionotificationportref?language=objc)"] pub type IONotificationPortRef = * mut IONotificationPort ;
};
}
