mkuse!{use core :: ptr ;}
mkuse!{use super :: super :: riscv :: imply_features ;}
mkuse!{use super :: auxvec ;}
mkuse!{use crate :: detect :: { Feature , bit , cache } ;}
mkitem!{const PR_RISCV_V_GET_CONTROL : libc :: c_int = 70 ;}
mkitem!{const PR_RISCV_V_VSTATE_CTRL_ON : libc :: c_int = 2 ;}
mkitem!{const PR_RISCV_V_VSTATE_CTRL_CUR_MASK : libc :: c_int = 3 ;}
mkitem!{mkstruct!{# [repr (C)] struct riscv_hwprobe { key : i64 , value : u64 , }}}
mkitem!{mkimpl!{impl riscv_hwprobe { pub fn get (& self) -> Option < u64 > { (self . key != - 1) . then_some (self . value) } }}}
mkitem!{# [allow (non_upper_case_globals)] const __NR_riscv_hwprobe : libc :: c_long = 258 ;}
mkitem!{const RISCV_HWPROBE_KEY_BASE_BEHAVIOR : i64 = 3 ;}
mkitem!{const RISCV_HWPROBE_BASE_BEHAVIOR_IMA : u64 = 1 << 0 ;}
mkitem!{const RISCV_HWPROBE_KEY_IMA_EXT_0 : i64 = 4 ;}
mkitem!{const RISCV_HWPROBE_IMA_FD : u64 = 1 << 0 ;}
mkitem!{const RISCV_HWPROBE_IMA_C : u64 = 1 << 1 ;}
mkitem!{const RISCV_HWPROBE_IMA_V : u64 = 1 << 2 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZBA : u64 = 1 << 3 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZBB : u64 = 1 << 4 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZBS : u64 = 1 << 5 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZICBOZ : u64 = 1 << 6 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZBC : u64 = 1 << 7 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZBKB : u64 = 1 << 8 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZBKC : u64 = 1 << 9 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZBKX : u64 = 1 << 10 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZKND : u64 = 1 << 11 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZKNE : u64 = 1 << 12 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZKNH : u64 = 1 << 13 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZKSED : u64 = 1 << 14 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZKSH : u64 = 1 << 15 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZKT : u64 = 1 << 16 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVBB : u64 = 1 << 17 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVBC : u64 = 1 << 18 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVKB : u64 = 1 << 19 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVKG : u64 = 1 << 20 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVKNED : u64 = 1 << 21 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVKNHA : u64 = 1 << 22 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVKNHB : u64 = 1 << 23 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVKSED : u64 = 1 << 24 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVKSH : u64 = 1 << 25 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVKT : u64 = 1 << 26 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZFH : u64 = 1 << 27 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZFHMIN : u64 = 1 << 28 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZIHINTNTL : u64 = 1 << 29 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVFH : u64 = 1 << 30 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVFHMIN : u64 = 1 << 31 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZFA : u64 = 1 << 32 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZTSO : u64 = 1 << 33 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZACAS : u64 = 1 << 34 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZICOND : u64 = 1 << 35 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZIHINTPAUSE : u64 = 1 << 36 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVE32X : u64 = 1 << 37 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVE32F : u64 = 1 << 38 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVE64X : u64 = 1 << 39 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVE64F : u64 = 1 << 40 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVE64D : u64 = 1 << 41 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZIMOP : u64 = 1 << 42 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZCA : u64 = 1 << 43 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZCB : u64 = 1 << 44 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZCD : u64 = 1 << 45 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZCF : u64 = 1 << 46 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZCMOP : u64 = 1 << 47 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZAWRS : u64 = 1 << 48 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZICNTR : u64 = 1 << 50 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZIHPM : u64 = 1 << 51 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZFBFMIN : u64 = 1 << 52 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVFBFMIN : u64 = 1 << 53 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZVFBFWMA : u64 = 1 << 54 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZICBOM : u64 = 1 << 55 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZAAMO : u64 = 1 << 56 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZALRSC : u64 = 1 << 57 ;}
mkitem!{const RISCV_HWPROBE_EXT_ZABHA : u64 = 1 << 58 ;}
mkitem!{const RISCV_HWPROBE_KEY_CPUPERF_0 : i64 = 5 ;}
mkitem!{const RISCV_HWPROBE_MISALIGNED_FAST : u64 = 3 ;}
mkitem!{const RISCV_HWPROBE_MISALIGNED_MASK : u64 = 7 ;}
mkitem!{const RISCV_HWPROBE_KEY_MISALIGNED_SCALAR_PERF : i64 = 9 ;}
mkitem!{const RISCV_HWPROBE_MISALIGNED_SCALAR_FAST : u64 = 3 ;}
mkitem!{const RISCV_HWPROBE_KEY_MISALIGNED_VECTOR_PERF : i64 = 10 ;}
mkitem!{const RISCV_HWPROBE_MISALIGNED_VECTOR_FAST : u64 = 3 ;}

macro_rules! _riscv_hwprobe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _riscv_hwprobe in module {}", module_path!());
    };
}

mkfn!{
    _riscv_hwprobe_introspect!();
    fn _riscv_hwprobe (out : & mut [riscv_hwprobe]) -> bool { unsafe fn __riscv_hwprobe (pairs : * mut riscv_hwprobe , pair_count : libc :: size_t , cpu_set_size : libc :: size_t , cpus : * mut libc :: c_ulong , flags : libc :: c_uint ,) -> libc :: c_long { unsafe { libc :: syscall (__NR_riscv_hwprobe , pairs , pair_count , cpu_set_size , cpus , flags) } } unsafe { __riscv_hwprobe (out . as_mut_ptr () , out . len () , 0 , ptr :: null_mut () , 0) == 0 } }
}

macro_rules! detect_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_features in module {}", module_path!());
    };
}

mkfn!{
    detect_features_introspect!();
    # [doc = " Read list of supported features from (1) the auxiliary vector"] # [doc = " and (2) the results of `riscv_hwprobe` and `prctl` system calls."] pub (crate) fn detect_features () -> cache :: Initializer { let mut value = cache :: Initializer :: default () ; let mut enable_feature = | feature , enable | { if enable { value . set (feature as u32) ; } } ; let auxv = auxvec :: auxv () . expect ("read auxvec") ; let mut has_i = bit :: test (auxv . hwcap , (b'i' - b'a') . into ()) ; # [allow (clippy :: eq_op)] enable_feature (Feature :: a , bit :: test (auxv . hwcap , (b'a' - b'a') . into ())) ; enable_feature (Feature :: c , bit :: test (auxv . hwcap , (b'c' - b'a') . into ())) ; enable_feature (Feature :: d , bit :: test (auxv . hwcap , (b'd' - b'a') . into ())) ; enable_feature (Feature :: f , bit :: test (auxv . hwcap , (b'f' - b'a') . into ())) ; enable_feature (Feature :: m , bit :: test (auxv . hwcap , (b'm' - b'a') . into ())) ; let has_v = bit :: test (auxv . hwcap , (b'v' - b'a') . into ()) ; let mut is_v_set = false ; 'hwprobe : { macro_rules ! init { { $ ($ name : ident : $ key : expr) ,* $ (,) ? } => { # [repr (usize)] enum Indices { $ ($ name) ,* } let mut t = [$ (riscv_hwprobe { key : $ key , value : 0 }) ,*] ; macro_rules ! data_mut { () => { & mut t } } macro_rules ! query { [$ idx : ident] => { t [Indices ::$ idx as usize] . get () } } } } init ! { BaseBehavior : RISCV_HWPROBE_KEY_BASE_BEHAVIOR , Extensions : RISCV_HWPROBE_KEY_IMA_EXT_0 , MisalignedScalarPerf : RISCV_HWPROBE_KEY_MISALIGNED_SCALAR_PERF , MisalignedVectorPerf : RISCV_HWPROBE_KEY_MISALIGNED_VECTOR_PERF , MisalignedScalarPerfFallback : RISCV_HWPROBE_KEY_CPUPERF_0 , } ; if ! _riscv_hwprobe (data_mut ! ()) { break 'hwprobe ; } if let Some (value) = query ! [MisalignedScalarPerf] { enable_feature (Feature :: unaligned_scalar_mem , value == RISCV_HWPROBE_MISALIGNED_SCALAR_FAST ,) ; } else if let Some (value) = query ! [MisalignedScalarPerfFallback] { enable_feature (Feature :: unaligned_scalar_mem , value & RISCV_HWPROBE_MISALIGNED_MASK == RISCV_HWPROBE_MISALIGNED_FAST ,) ; } if let Some (value) = query ! [MisalignedVectorPerf] { enable_feature (Feature :: unaligned_vector_mem , value == RISCV_HWPROBE_MISALIGNED_VECTOR_FAST ,) ; } if query ! [BaseBehavior] . is_none_or (| value | value & RISCV_HWPROBE_BASE_BEHAVIOR_IMA == 0) { break 'hwprobe ; } has_i = true ; enable_feature (Feature :: zicsr , true) ; enable_feature (Feature :: zicntr , true) ; enable_feature (Feature :: zifencei , true) ; enable_feature (Feature :: m , true) ; enable_feature (Feature :: a , true) ; let Some (ima_ext_0) = query ! [Extensions] else { break 'hwprobe ; } ; let test = | mask | (ima_ext_0 & mask) != 0 ; enable_feature (Feature :: d , test (RISCV_HWPROBE_IMA_FD)) ; enable_feature (Feature :: c , test (RISCV_HWPROBE_IMA_C)) ; enable_feature (Feature :: zicntr , test (RISCV_HWPROBE_EXT_ZICNTR)) ; enable_feature (Feature :: zihpm , test (RISCV_HWPROBE_EXT_ZIHPM)) ; enable_feature (Feature :: zihintntl , test (RISCV_HWPROBE_EXT_ZIHINTNTL)) ; enable_feature (Feature :: zihintpause , test (RISCV_HWPROBE_EXT_ZIHINTPAUSE)) ; enable_feature (Feature :: zimop , test (RISCV_HWPROBE_EXT_ZIMOP)) ; enable_feature (Feature :: zicbom , test (RISCV_HWPROBE_EXT_ZICBOM)) ; enable_feature (Feature :: zicboz , test (RISCV_HWPROBE_EXT_ZICBOZ)) ; enable_feature (Feature :: zicond , test (RISCV_HWPROBE_EXT_ZICOND)) ; enable_feature (Feature :: zalrsc , test (RISCV_HWPROBE_EXT_ZALRSC)) ; enable_feature (Feature :: zaamo , test (RISCV_HWPROBE_EXT_ZAAMO)) ; enable_feature (Feature :: zawrs , test (RISCV_HWPROBE_EXT_ZAWRS)) ; enable_feature (Feature :: zabha , test (RISCV_HWPROBE_EXT_ZABHA)) ; enable_feature (Feature :: zacas , test (RISCV_HWPROBE_EXT_ZACAS)) ; enable_feature (Feature :: ztso , test (RISCV_HWPROBE_EXT_ZTSO)) ; enable_feature (Feature :: zba , test (RISCV_HWPROBE_EXT_ZBA)) ; enable_feature (Feature :: zbb , test (RISCV_HWPROBE_EXT_ZBB)) ; enable_feature (Feature :: zbs , test (RISCV_HWPROBE_EXT_ZBS)) ; enable_feature (Feature :: zbc , test (RISCV_HWPROBE_EXT_ZBC)) ; enable_feature (Feature :: zbkb , test (RISCV_HWPROBE_EXT_ZBKB)) ; enable_feature (Feature :: zbkc , test (RISCV_HWPROBE_EXT_ZBKC)) ; enable_feature (Feature :: zbkx , test (RISCV_HWPROBE_EXT_ZBKX)) ; enable_feature (Feature :: zknd , test (RISCV_HWPROBE_EXT_ZKND)) ; enable_feature (Feature :: zkne , test (RISCV_HWPROBE_EXT_ZKNE)) ; enable_feature (Feature :: zknh , test (RISCV_HWPROBE_EXT_ZKNH)) ; enable_feature (Feature :: zksed , test (RISCV_HWPROBE_EXT_ZKSED)) ; enable_feature (Feature :: zksh , test (RISCV_HWPROBE_EXT_ZKSH)) ; enable_feature (Feature :: zkt , test (RISCV_HWPROBE_EXT_ZKT)) ; enable_feature (Feature :: zcmop , test (RISCV_HWPROBE_EXT_ZCMOP)) ; enable_feature (Feature :: zca , test (RISCV_HWPROBE_EXT_ZCA)) ; enable_feature (Feature :: zcf , test (RISCV_HWPROBE_EXT_ZCF)) ; enable_feature (Feature :: zcd , test (RISCV_HWPROBE_EXT_ZCD)) ; enable_feature (Feature :: zcb , test (RISCV_HWPROBE_EXT_ZCB)) ; enable_feature (Feature :: zfh , test (RISCV_HWPROBE_EXT_ZFH)) ; enable_feature (Feature :: zfhmin , test (RISCV_HWPROBE_EXT_ZFHMIN)) ; enable_feature (Feature :: zfa , test (RISCV_HWPROBE_EXT_ZFA)) ; enable_feature (Feature :: zfbfmin , test (RISCV_HWPROBE_EXT_ZFBFMIN)) ; let has_vectors = { let v_status = unsafe { libc :: prctl (PR_RISCV_V_GET_CONTROL) } ; if v_status >= 0 { (v_status & PR_RISCV_V_VSTATE_CTRL_CUR_MASK) == PR_RISCV_V_VSTATE_CTRL_ON } else { has_v } } ; if has_vectors { enable_feature (Feature :: v , test (RISCV_HWPROBE_IMA_V)) ; enable_feature (Feature :: zve32x , test (RISCV_HWPROBE_EXT_ZVE32X)) ; enable_feature (Feature :: zve32f , test (RISCV_HWPROBE_EXT_ZVE32F)) ; enable_feature (Feature :: zve64x , test (RISCV_HWPROBE_EXT_ZVE64X)) ; enable_feature (Feature :: zve64f , test (RISCV_HWPROBE_EXT_ZVE64F)) ; enable_feature (Feature :: zve64d , test (RISCV_HWPROBE_EXT_ZVE64D)) ; enable_feature (Feature :: zvbb , test (RISCV_HWPROBE_EXT_ZVBB)) ; enable_feature (Feature :: zvbc , test (RISCV_HWPROBE_EXT_ZVBC)) ; enable_feature (Feature :: zvkb , test (RISCV_HWPROBE_EXT_ZVKB)) ; enable_feature (Feature :: zvkg , test (RISCV_HWPROBE_EXT_ZVKG)) ; enable_feature (Feature :: zvkned , test (RISCV_HWPROBE_EXT_ZVKNED)) ; enable_feature (Feature :: zvknha , test (RISCV_HWPROBE_EXT_ZVKNHA)) ; enable_feature (Feature :: zvknhb , test (RISCV_HWPROBE_EXT_ZVKNHB)) ; enable_feature (Feature :: zvksed , test (RISCV_HWPROBE_EXT_ZVKSED)) ; enable_feature (Feature :: zvksh , test (RISCV_HWPROBE_EXT_ZVKSH)) ; enable_feature (Feature :: zvkt , test (RISCV_HWPROBE_EXT_ZVKT)) ; enable_feature (Feature :: zvfh , test (RISCV_HWPROBE_EXT_ZVFH)) ; enable_feature (Feature :: zvfhmin , test (RISCV_HWPROBE_EXT_ZVFHMIN)) ; enable_feature (Feature :: zvfbfmin , test (RISCV_HWPROBE_EXT_ZVFBFMIN)) ; enable_feature (Feature :: zvfbfwma , test (RISCV_HWPROBE_EXT_ZVFBFWMA)) ; } is_v_set = true ; } ; if ! is_v_set { enable_feature (Feature :: v , has_v) ; } # [cfg (target_arch = "riscv64")] enable_feature (Feature :: rv64i , has_i) ; # [cfg (target_arch = "riscv32")] enable_feature (Feature :: rv32i , has_i) ; imply_features (value) }
}