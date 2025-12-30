// Generated macro for GenOpts (struct)
macro_rules! Depcrate_generateGenOpts {
() => {
// Module: crate::generate
// Provides: {"GenOpts"}
// Dependencies: {}
# [doc = " Code generation options"] # [derive (Clone , Debug)] pub struct GenOpts { # [doc = " Name of dbus crate (used for import)"] pub dbuscrate : String , # [doc = " MethodType for dbus-tree impl, set to none for client impl only"] pub methodtype : Option < String > , # [doc = " Generate dbus-crossroads server implementation"] pub crossroads : bool , # [doc = " Removes a prefix from interface names"] pub skipprefix : Option < String > , # [doc = " Type of server access (tree)"] pub serveraccess : ServerAccess , # [doc = " Tries to make variants generic instead of Variant<Box<Refarg>>"] pub genericvariant : bool , # [doc = " Type of connection, for client only"] pub connectiontype : ConnectionType , # [doc = " Generates a struct wrapping PropMap to get properties from it with their expected types."] pub propnewtype : bool , # [doc = " interface filter. Only matching interface are generated, if non-empty."] pub interfaces : Option < HashSet < String > > , # [doc = " The command line argument string. This will be inserted into generated source files."] pub command_line : String , }
};
}
