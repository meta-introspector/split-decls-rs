// Generated macro for introspect (function)
macro_rules! Depcrate_stdimplintrospect {
() => {
// Module: crate::stdimpl
// Provides: {"introspect"}
// Dependencies: {}
fn introspect (cr : & Crossroads , path : & dbus :: Path < 'static >) -> String { let mut children = cr . get_children (path , true) ; let mut childstr = String :: new () ; children . sort_unstable () ; for c in children { childstr += & format ! ("  <node name=\"{}\"/>\n" , c) ; } let (reg , ifaces) = cr . registry_and_ifaces (path) ; let ifacestr = reg . introspect (ifaces) ; let nodestr = format ! (r##"<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN"
 "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node name="{}">
{}{}</node>"## , path , ifacestr , childstr) ; nodestr }
};
}
