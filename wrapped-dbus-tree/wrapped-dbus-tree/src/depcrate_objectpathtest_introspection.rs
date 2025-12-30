// Generated macro for test_introspection (function)
macro_rules! Depcrate_objectpathtest_introspection {
() => {
// Module: crate::objectpath
// Provides: {"test_introspection"}
// Dependencies: {}
# [test] fn test_introspection () { let f = super :: Factory :: new_fn :: < () > () ; let t = f . object_path ("/echo" , ()) . introspectable () . add (f . interface ("com.example.echo" , ()) . add_m (f . method ("Echo" , () , | _ | unimplemented ! ()) . in_arg (("request" , "s")) . out_arg (("reply" , "s"))) . add_p (f . property :: < i32 , _ > ("EchoCount" , ())) . add_s (f . signal ("Echoed" , ()) . arg (("data" , "s")) . deprecated ())) ; let actual_result = t . introspect (& f . tree (()) . add (f . object_path ("/echo/subpath2" , ())) . add (f . object_path ("/echo/subpath" , ()))) ; println ! ("\n=== Introspection XML start ===\n{}\n=== Introspection XML end ===" , actual_result) ; let expected_result = r##"<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN" "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node name="/echo">
  <interface name="com.example.echo">
    <method name="Echo">
      <arg name="request" type="s" direction="in"/>
      <arg name="reply" type="s" direction="out"/>
    </method>
    <property name="EchoCount" type="i" access="read"/>
    <signal name="Echoed">
      <arg name="data" type="s"/>
      <annotation name="org.freedesktop.DBus.Deprecated" value="true"/>
    </signal>
  </interface>
  <interface name="org.freedesktop.DBus.Introspectable">
    <method name="Introspect">
      <arg name="xml_data" type="s" direction="out"/>
    </method>
  </interface>
  <interface name="org.freedesktop.DBus.Properties">
    <method name="Get">
      <arg name="interface_name" type="s" direction="in"/>
      <arg name="property_name" type="s" direction="in"/>
      <arg name="value" type="v" direction="out"/>
    </method>
    <method name="GetAll">
      <arg name="interface_name" type="s" direction="in"/>
      <arg name="props" type="a{sv}" direction="out"/>
    </method>
    <method name="Set">
      <arg name="interface_name" type="s" direction="in"/>
      <arg name="property_name" type="s" direction="in"/>
      <arg name="value" type="v" direction="in"/>
    </method>
    <signal name="PropertiesChanged">
      <arg name="interface_name" type="s"/>
      <arg name="changed_properties" type="a{sv}"/>
      <arg name="invalidated_properties" type="as"/>
    </signal>
  </interface>
  <node name="subpath"/>
  <node name="subpath2"/>
</node>"## ; assert_eq ! (expected_result , actual_result) ; }
};
}
