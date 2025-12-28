macro_rules! deps {
    () => {
        U32Bytes!();
    };
}

macro_rules! ImageImportDescriptor {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageImportDescriptor { # [doc = " RVA to original unbound IAT (`ImageThunkData32`/`ImageThunkData64`)"] # [doc = " 0 for terminating null import descriptor"] pub original_first_thunk : U32Bytes < LE > , # [doc = " 0 if not bound,"] # [doc = " -1 if bound, and real date\\time stamp"] # [doc = "     in IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT (new BIND)"] # [doc = " O.W. date/time stamp of DLL bound to (Old BIND)"] pub time_date_stamp : U32Bytes < LE > , # [doc = " -1 if no forwarders"] pub forwarder_chain : U32Bytes < LE > , pub name : U32Bytes < LE > , # [doc = " RVA to IAT (if bound this IAT has actual addresses)"] pub first_thunk : U32Bytes < LE > , }
    };
}

ImageImportDescriptor!();