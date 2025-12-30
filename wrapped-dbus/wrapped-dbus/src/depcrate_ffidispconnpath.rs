// Generated macro for ConnPath (struct)
macro_rules! Depcrate_ffidispConnPath {
() => {
// Module: crate::ffidisp
// Provides: {"ConnPath"}
// Dependencies: {}
# [doc = " A convenience struct that wraps connection, destination and path."] # [doc = ""] # [doc = " Useful if you want to make many method calls to the same destination path."] # [derive (Clone , Debug)] pub struct ConnPath < 'a , C > { # [doc = " Some way to access the connection, e g a &Connection or Rc<Connection>"] pub conn : C , # [doc = " Destination, i e what D-Bus service you're communicating with"] pub dest : BusName < 'a > , # [doc = " Object path on the destination"] pub path : Path < 'a > , # [doc = " Timeout in milliseconds for blocking method calls"] pub timeout : i32 , }
};
}
