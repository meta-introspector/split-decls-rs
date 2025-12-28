macro_rules! deps {
    () => {
        IsExecutable!();
    };
}

macro_rules! unix {
    () => {
        deps!();
        # [cfg (unix)] mod unix { use std :: os :: unix :: fs :: PermissionsExt ; use std :: path :: Path ; use super :: IsExecutable ; impl IsExecutable for Path { fn is_executable (& self) -> bool { let metadata = match self . metadata () { Ok (metadata) => metadata , Err (_) => return false , } ; let permissions = metadata . permissions () ; metadata . is_file () && permissions . mode () & 0o111 != 0 } } }
    };
}

unix!()