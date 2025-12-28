macro_rules! ReseedingCore {
    () => {
        # [derive (Debug)] struct ReseedingCore < R , Rsdr > { inner : R , reseeder : Rsdr , threshold : i64 , bytes_until_reseed : i64 , }
    };
}

ReseedingCore!();