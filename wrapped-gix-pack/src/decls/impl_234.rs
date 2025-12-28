macro_rules! deps {
    () => {
        Reducer!();
        SafetyCheck!();
        Statistics!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'a , P , E > Reducer < 'a , P , E > where P : Progress , { pub fn from_progress (progress : OwnShared < Mutable < P > > , pack_data_len_in_bytes : usize , check : traverse :: SafetyCheck , should_interrupt : & 'a AtomicBool ,) -> Self { let stats = traverse :: Statistics { pack_size : pack_data_len_in_bytes as u64 , .. Default :: default () } ; Reducer { progress , check , then : Instant :: now () , entries_seen : 0 , should_interrupt , stats , _error : Default :: default () , } } }
    };
}

impl_234!();