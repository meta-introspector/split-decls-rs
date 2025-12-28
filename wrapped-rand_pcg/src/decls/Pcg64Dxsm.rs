macro_rules! deps {
    () => {
        Lcg128CmDxsm64!();
    };
}

macro_rules! Pcg64Dxsm {
    () => {
        deps!();
        # [doc = " [`Lcg128CmDxsm64`] is also known as `PCG64DXSM`."] pub type Pcg64Dxsm = Lcg128CmDxsm64 ;
    };
}

Pcg64Dxsm!();