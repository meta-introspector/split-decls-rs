macro_rules! deps {
    () => {
        Accels!();
        AccelTy!();
        DeserializeError!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'a > Accels < & 'a [AccelTy] > { # [doc = " Deserialize a sequence of accelerators from the given bytes. If there"] # [doc = " was a problem deserializing, then an error is returned."] # [doc = ""] # [doc = " This is guaranteed to run in constant time. This does not guarantee"] # [doc = " that every accelerator in the returned collection is valid. Thus,"] # [doc = " accessing one may panic, or not-safe code that relies on accelerators"] # [doc = " being correct my result in UB."] # [doc = ""] # [doc = " Callers may check the validity of every accelerator with the `validate`"] # [doc = " method."] pub fn from_bytes_unchecked (mut slice : & 'a [u8] ,) -> Result < (Accels < & 'a [AccelTy] > , usize) , DeserializeError > { let slice_start = slice . as_ptr () . as_usize () ; let (accel_len , _) = wire :: try_read_u32_as_usize (slice , "accelerators length") ? ; let accel_tys_len = wire :: add (wire :: mul (accel_len , 2 , "total number of accelerator accel_tys") ? , 1 , "total number of accel_tys" ,) ? ; let accel_tys_bytes_len = wire :: mul (ACCEL_TY_SIZE , accel_tys_len , "total number of bytes in accelerators" ,) ? ; wire :: check_slice_len (slice , accel_tys_bytes_len , "accelerators") ? ; wire :: check_alignment :: < AccelTy > (slice) ? ; let accel_tys = & slice [.. accel_tys_bytes_len] ; slice = & slice [accel_tys_bytes_len ..] ; let accels = unsafe { core :: slice :: from_raw_parts (accel_tys . as_ptr () . cast :: < AccelTy > () , accel_tys_len ,) } ; Ok ((Accels { accels } , slice . as_ptr () . as_usize () - slice_start)) } }
    };
}

impl_160!()