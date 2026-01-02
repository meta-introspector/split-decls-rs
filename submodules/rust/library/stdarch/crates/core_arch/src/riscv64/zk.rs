mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.riscv.aes64es"] fn _aes64es (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.aes64esm"] fn _aes64esm (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.aes64ds"] fn _aes64ds (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.aes64dsm"] fn _aes64dsm (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.aes64ks1i"] fn _aes64ks1i (rs1 : i64 , rnum : i32) -> i64 ; # [link_name = "llvm.riscv.aes64ks2"] fn _aes64ks2 (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.aes64im"] fn _aes64im (rs1 : i64) -> i64 ; # [link_name = "llvm.riscv.sha512sig0"] fn _sha512sig0 (rs1 : i64) -> i64 ; # [link_name = "llvm.riscv.sha512sig1"] fn _sha512sig1 (rs1 : i64) -> i64 ; # [link_name = "llvm.riscv.sha512sum0"] fn _sha512sum0 (rs1 : i64) -> i64 ; # [link_name = "llvm.riscv.sha512sum1"] fn _sha512sum1 (rs1 : i64) -> i64 ; }}

macro_rules! aes64es_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes64es in module {}", module_path!());
    };
}

mkfn!{
    aes64es_introspect!();
    # [doc = " AES final round encryption instruction for RV64."] # [doc = ""] # [doc = " Uses the two 64-bit source registers to represent the entire AES state, and produces half"] # [doc = " of the next round output, applying the ShiftRows and SubBytes steps. This instruction must"] # [doc = " always be implemented such that its execution latency does not depend on the data being"] # [doc = " operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.7"] # [target_feature (enable = "zkne")] # [cfg_attr (test , assert_instr (aes64es))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes64es (rs1 : u64 , rs2 : u64) -> u64 { unsafe { _aes64es (rs1 as i64 , rs2 as i64) as u64 } }
}

macro_rules! aes64esm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes64esm in module {}", module_path!());
    };
}

mkfn!{
    aes64esm_introspect!();
    # [doc = " AES middle round encryption instruction for RV64."] # [doc = ""] # [doc = " Uses the two 64-bit source registers to represent the entire AES state, and produces half"] # [doc = " of the next round output, applying the ShiftRows, SubBytes and MixColumns steps. This"] # [doc = " instruction must always be implemented such that its execution latency does not depend on"] # [doc = " the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.8"] # [target_feature (enable = "zkne")] # [cfg_attr (test , assert_instr (aes64esm))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes64esm (rs1 : u64 , rs2 : u64) -> u64 { unsafe { _aes64esm (rs1 as i64 , rs2 as i64) as u64 } }
}

macro_rules! aes64ds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes64ds in module {}", module_path!());
    };
}

mkfn!{
    aes64ds_introspect!();
    # [doc = " AES final round decryption instruction for RV64."] # [doc = ""] # [doc = " Uses the two 64-bit source registers to represent the entire AES state, and produces half"] # [doc = " of the next round output, applying the Inverse ShiftRows and SubBytes steps. This"] # [doc = " instruction must always be implemented such that its execution latency does not depend on"] # [doc = " the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.5"] # [target_feature (enable = "zknd")] # [cfg_attr (test , assert_instr (aes64ds))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes64ds (rs1 : u64 , rs2 : u64) -> u64 { unsafe { _aes64ds (rs1 as i64 , rs2 as i64) as u64 } }
}

macro_rules! aes64dsm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes64dsm in module {}", module_path!());
    };
}

mkfn!{
    aes64dsm_introspect!();
    # [doc = " AES middle round decryption instruction for RV64."] # [doc = ""] # [doc = " Uses the two 64-bit source registers to represent the entire AES state, and produces half"] # [doc = " of the next round output, applying the Inverse ShiftRows, SubBytes and MixColumns steps."] # [doc = " This instruction must always be implemented such that its execution latency does not depend"] # [doc = " on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.6"] # [target_feature (enable = "zknd")] # [cfg_attr (test , assert_instr (aes64dsm))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes64dsm (rs1 : u64 , rs2 : u64) -> u64 { unsafe { _aes64dsm (rs1 as i64 , rs2 as i64) as u64 } }
}

macro_rules! aes64ks1i_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes64ks1i in module {}", module_path!());
    };
}

mkfn!{
    aes64ks1i_introspect!();
    # [doc = " This instruction implements part of the KeySchedule operation for the AES Block cipher"] # [doc = " involving the SBox operation."] # [doc = ""] # [doc = " This instruction implements the rotation, SubBytes and Round Constant addition steps of the"] # [doc = " AES block cipher Key Schedule. This instruction must always be implemented such that its"] # [doc = " execution latency does not depend on the data being operated on. Note that rnum must be in"] # [doc = " the range 0x0..0xA. The values 0xB..0xF are reserved."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.10"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `RNUM` parameter is expected to be a constant value inside the range of `0..=10`."] # [target_feature (enable = "zkne" , enable = "zknd")] # [rustc_legacy_const_generics (1)] # [cfg_attr (test , assert_instr (aes64ks1i , RNUM = 0))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes64ks1i < const RNUM : u8 > (rs1 : u64) -> u64 { static_assert ! (RNUM <= 10) ; unsafe { _aes64ks1i (rs1 as i64 , RNUM as i32) as u64 } }
}

macro_rules! aes64ks2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes64ks2 in module {}", module_path!());
    };
}

mkfn!{
    aes64ks2_introspect!();
    # [doc = " This instruction implements part of the KeySchedule operation for the AES Block cipher."] # [doc = ""] # [doc = " This instruction implements the additional XOR’ing of key words as part of the AES block"] # [doc = " cipher Key Schedule. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.11"] # [target_feature (enable = "zkne" , enable = "zknd")] # [cfg_attr (test , assert_instr (aes64ks2))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes64ks2 (rs1 : u64 , rs2 : u64) -> u64 { unsafe { _aes64ks2 (rs1 as i64 , rs2 as i64) as u64 } }
}

macro_rules! aes64im_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes64im in module {}", module_path!());
    };
}

mkfn!{
    aes64im_introspect!();
    # [doc = " This instruction accelerates the inverse MixColumns step of the AES Block Cipher, and is used to aid creation of"] # [doc = " the decryption KeySchedule."] # [doc = ""] # [doc = " The instruction applies the inverse MixColumns transformation to two columns of the state array, packed"] # [doc = " into a single 64-bit register. It is used to create the inverse cipher KeySchedule, according to the equivalent"] # [doc = " inverse cipher construction in (Page 23, Section 5.3.5). This instruction must always be implemented"] # [doc = " such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.9"] # [target_feature (enable = "zkne" , enable = "zknd")] # [cfg_attr (test , assert_instr (aes64im))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes64im (rs1 : u64) -> u64 { unsafe { _aes64im (rs1 as i64) as u64 } }
}

macro_rules! sha512sig0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sig0 in module {}", module_path!());
    };
}

mkfn!{
    sha512sig0_introspect!();
    # [doc = " Implements the Sigma0 transformation function as used in the SHA2-512 hash function \\[49\\]"] # [doc = " (Section 4.1.3)."] # [doc = ""] # [doc = " This instruction is supported for the RV64 base architecture. It implements the Sigma0"] # [doc = " transform of the SHA2-512 hash function. \\[49\\]. This instruction must always be"] # [doc = " implemented such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.37"] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha512sig0))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sig0 (rs1 : u64) -> u64 { unsafe { _sha512sig0 (rs1 as i64) as u64 } }
}

macro_rules! sha512sig1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sig1 in module {}", module_path!());
    };
}

mkfn!{
    sha512sig1_introspect!();
    # [doc = " Implements the Sigma1 transformation function as used in the SHA2-512 hash function \\[49\\]"] # [doc = " (Section 4.1.3)."] # [doc = ""] # [doc = " This instruction is supported for the RV64 base architecture. It implements the Sigma1"] # [doc = " transform of the SHA2-512 hash function. \\[49\\]. This instruction must always be"] # [doc = " implemented such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.38"] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha512sig1))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sig1 (rs1 : u64) -> u64 { unsafe { _sha512sig1 (rs1 as i64) as u64 } }
}

macro_rules! sha512sum0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sum0 in module {}", module_path!());
    };
}

mkfn!{
    sha512sum0_introspect!();
    # [doc = " Implements the Sum0 transformation function as used in the SHA2-512 hash function \\[49\\]"] # [doc = " (Section 4.1.3)."] # [doc = ""] # [doc = " This instruction is supported for the RV64 base architecture. It implements the Sum0"] # [doc = " transform of the SHA2-512 hash function. \\[49\\]. This instruction must always be"] # [doc = " implemented such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.39"] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha512sum0))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sum0 (rs1 : u64) -> u64 { unsafe { _sha512sum0 (rs1 as i64) as u64 } }
}

macro_rules! sha512sum1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sum1 in module {}", module_path!());
    };
}

mkfn!{
    sha512sum1_introspect!();
    # [doc = " Implements the Sum1 transformation function as used in the SHA2-512 hash function \\[49\\]"] # [doc = " (Section 4.1.3)."] # [doc = ""] # [doc = " This instruction is supported for the RV64 base architecture. It implements the Sum1"] # [doc = " transform of the SHA2-512 hash function. \\[49\\]. This instruction must always be"] # [doc = " implemented such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.40"] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha512sum1))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sum1 (rs1 : u64) -> u64 { unsafe { _sha512sum1 (rs1 as i64) as u64 } }
}