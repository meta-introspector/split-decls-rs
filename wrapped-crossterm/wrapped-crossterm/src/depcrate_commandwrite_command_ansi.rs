// Generated macro for write_command_ansi (function)
macro_rules! Depcrate_commandwrite_command_ansi {
() => {
// Module: crate::command
// Provides: {"write_command_ansi"}
// Dependencies: {}
# [doc = " Writes the ANSI representation of a command to the given writer."] fn write_command_ansi < C : Command > (io : & mut (impl io :: Write + ? Sized) , command : C ,) -> io :: Result < () > { struct Adapter < T > { inner : T , res : io :: Result < () > , } impl < T : Write > fmt :: Write for Adapter < T > { fn write_str (& mut self , s : & str) -> fmt :: Result { self . inner . write_all (s . as_bytes ()) . map_err (| e | { self . res = Err (e) ; fmt :: Error }) } } let mut adapter = Adapter { inner : io , res : Ok (()) , } ; command . write_ansi (& mut adapter) . map_err (| fmt :: Error | match adapter . res { Ok (()) => panic ! ("<{}>::write_ansi incorrectly errored" , std :: any :: type_name ::< C > ()) , Err (e) => e , }) }
};
}
