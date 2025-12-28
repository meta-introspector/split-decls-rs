macro_rules! deps {
    () => {
        OwnedKey!();
    };
}

macro_rules! Repr {
    () => {
        deps!();
        # [doc = " Note: must not encode `HKEY_PERFORMANCE_DATA` or one of its subkeys."] enum Repr { # [doc = " `HKEY_LOCAL_MACHINE`."] LocalMachine , # [doc = " A subkey of `HKEY_LOCAL_MACHINE`."] Owned (OwnedKey) , }
    };
}

Repr!();