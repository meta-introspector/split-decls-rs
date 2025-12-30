// Generated macro for test_introspection_dynamic (function)
macro_rules! Depcrate_objectpathtest_introspection_dynamic {
() => {
// Module: crate::objectpath
// Provides: {"test_introspection_dynamic"}
// Dependencies: {}
# [test] fn test_introspection_dynamic () { let f = super :: Factory :: new_fn :: < () > () ; let tree = f . tree (()) . add (f . object_path ("/" , ()) . introspectable ()) . add (f . object_path ("/foo/bar" , ()) . introspectable ()) . add (f . object_path ("/foo/bar/item1" , ()) . introspectable ()) ; let o = f . object_path ("/" , ()) . introspectable () ; let actual_result = o . introspect (& tree) ; println ! ("\n=== Introspection XML start ===\n{}\n=== Introspection XML end ===" , actual_result) ; let expected_result = r##"<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN" "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node name="/">
  <interface name="org.freedesktop.DBus.Introspectable">
    <method name="Introspect">
      <arg name="xml_data" type="s" direction="out"/>
    </method>
  </interface>
  <node name="foo/bar"/>
</node>"## ; assert_eq ! (expected_result , actual_result) ; let o = f . object_path ("/foo/bar" , ()) . introspectable () ; let actual_result = o . introspect (& tree) ; println ! ("\n=== Introspection XML start ===\n{}\n=== Introspection XML end ===" , actual_result) ; let expected_result = r##"<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN" "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node name="/foo/bar">
  <interface name="org.freedesktop.DBus.Introspectable">
    <method name="Introspect">
      <arg name="xml_data" type="s" direction="out"/>
    </method>
  </interface>
  <node name="item1"/>
</node>"## ; assert_eq ! (expected_result , actual_result) ; let tree = tree . add (f . object_path ("/foo/bar/item2" , ()) . introspectable ()) ; let o = f . object_path ("/" , ()) . introspectable () ; let actual_result = o . introspect (& tree) ; println ! ("\n=== Introspection XML start ===\n{}\n=== Introspection XML end ===" , actual_result) ; let expected_result = r##"<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN" "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node name="/">
  <interface name="org.freedesktop.DBus.Introspectable">
    <method name="Introspect">
      <arg name="xml_data" type="s" direction="out"/>
    </method>
  </interface>
  <node name="foo/bar"/>
</node>"## ; assert_eq ! (expected_result , actual_result) ; let o = f . object_path ("/foo/bar" , ()) . introspectable () ; let actual_result = o . introspect (& tree) ; println ! ("\n=== Introspection XML start ===\n{}\n=== Introspection XML end ===" , actual_result) ; let expected_result = r##"<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN" "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node name="/foo/bar">
  <interface name="org.freedesktop.DBus.Introspectable">
    <method name="Introspect">
      <arg name="xml_data" type="s" direction="out"/>
    </method>
  </interface>
  <node name="item1"/>
  <node name="item2"/>
</node>"## ; assert_eq ! (expected_result , actual_result) ; }
};
}
