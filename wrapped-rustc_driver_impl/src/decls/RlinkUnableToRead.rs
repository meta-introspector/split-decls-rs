macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! RlinkUnableToRead {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (driver_impl_rlink_unable_to_read)] pub (crate) struct RlinkUnableToRead { pub err : std :: io :: Error , }
    };
}

RlinkUnableToRead!();