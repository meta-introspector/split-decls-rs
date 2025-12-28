macro_rules! deps {
    () => {
        IsExecutable!();
    };
}

macro_rules! wasm {
    () => {
        deps!();
        # [cfg (any (target_os = "wasi" , target_family = "wasm"))] mod wasm { use std :: path :: Path ; use super :: IsExecutable ; impl IsExecutable for Path { fn is_executable (& self) -> bool { false } } }
    };
}

wasm!();