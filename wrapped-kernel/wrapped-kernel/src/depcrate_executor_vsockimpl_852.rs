// Generated macro for impl_852 (impl)
macro_rules! Depcrate_executor_vsockimpl_852 {
() => {
// Module: crate::executor::vsock
// Provides: {"impl_852"}
// Dependencies: {}
impl VsockMap { pub const fn new () -> Self { Self { port_map : BTreeMap :: new () , } } pub fn bind (& mut self , port : u32) -> io :: Result < () > { let entry = self . port_map . entry (port) ; match entry { btree_map :: Entry :: Vacant (vacant_entry) => { vacant_entry . insert (RawSocket :: new (VsockState :: Listen)) ; Ok (()) } btree_map :: Entry :: Occupied (_occupied_entry) => Err (Errno :: Addrinuse) , } } pub fn connect (& mut self , port : u32 , cid : u32) -> io :: Result < u32 > { for i in u32 :: MAX / 4 .. u32 :: MAX { let mut raw = RawSocket :: new (VsockState :: Connecting) ; raw . remote_cid = cid ; raw . remote_port = port ; if let btree_map :: Entry :: Vacant (vacant_entry) = self . port_map . entry (i) { vacant_entry . insert (raw) ; return Ok (i) ; } } Err (Errno :: Badf) } pub fn get_socket (& self , port : u32) -> Option < & RawSocket > { self . port_map . get (& port) } pub fn get_mut_socket (& mut self , port : u32) -> Option < & mut RawSocket > { self . port_map . get_mut (& port) } pub fn remove_socket (& mut self , port : u32) { let _ = self . port_map . remove (& port) ; } }
};
}
