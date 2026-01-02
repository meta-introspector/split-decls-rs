mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.riscv.sm4ed"] fn _sm4ed (rs1 : i32 , rs2 : i32 , bs : i32) -> i32 ; # [link_name = "llvm.riscv.sm4ks"] fn _sm4ks (rs1 : i32 , rs2 : i32 , bs : i32) -> i32 ; # [link_name = "llvm.riscv.sm3p0"] fn _sm3p0 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sm3p1"] fn _sm3p1 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha256sig0"] fn _sha256sig0 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha256sig1"] fn _sha256sig1 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha256sum0"] fn _sha256sum0 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha256sum1"] fn _sha256sum1 (rs1 : i32) -> i32 ; }}
mkitem!{# [cfg (target_arch = "riscv32")] unsafe extern "unadjusted" { # [link_name = "llvm.riscv.xperm8.i32"] fn _xperm8_32 (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.xperm4.i32"] fn _xperm4_32 (rs1 : i32 , rs2 : i32) -> i32 ; }}
mkitem!{# [cfg (target_arch = "riscv64")] unsafe extern "unadjusted" { # [link_name = "llvm.riscv.xperm8.i64"] fn _xperm8_64 (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.xperm4.i64"] fn _xperm4_64 (rs1 : i64 , rs2 : i64) -> i64 ; }}

macro_rules! xperm8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xperm8 in module {}", module_path!());
    };
}

mkfn!{
    xperm8_introspect!();
    # [doc = " Byte-wise lookup of indicies into a vector in registers."] # [doc = ""] # [doc = " The xperm8 instruction operates on bytes. The rs1 register contains a vector of XLEN/8"] # [doc = " 8-bit elements. The rs2 register contains a vector of XLEN/8 8-bit indexes. The result is"] # [doc = " each element in rs2 replaced by the indexed element in rs1, or zero if the index into rs2"] # [doc = " is out of bounds."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.47"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zbkx")] # [cfg_attr (test , assert_instr (xperm8))] # [inline] pub fn xperm8 (rs1 : usize , rs2 : usize) -> usize { # [cfg (target_arch = "riscv32")] unsafe { _xperm8_32 (rs1 as i32 , rs2 as i32) as usize } # [cfg (target_arch = "riscv64")] unsafe { _xperm8_64 (rs1 as i64 , rs2 as i64) as usize } }
}

macro_rules! xperm4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xperm4 in module {}", module_path!());
    };
}

mkfn!{
    xperm4_introspect!();
    # [doc = " Nibble-wise lookup of indicies into a vector."] # [doc = ""] # [doc = " The xperm4 instruction operates on nibbles. The rs1 register contains a vector of XLEN/4"] # [doc = " 4-bit elements. The rs2 register contains a vector of XLEN/4 4-bit indexes. The result is"] # [doc = " each element in rs2 replaced by the indexed element in rs1, or zero if the index into rs2"] # [doc = " is out of bounds."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.48"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zbkx")] # [cfg_attr (test , assert_instr (xperm4))] # [inline] pub fn xperm4 (rs1 : usize , rs2 : usize) -> usize { # [cfg (target_arch = "riscv32")] unsafe { _xperm4_32 (rs1 as i32 , rs2 as i32) as usize } # [cfg (target_arch = "riscv64")] unsafe { _xperm4_64 (rs1 as i64 , rs2 as i64) as usize } }
}

macro_rules! sha256sig0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha256sig0 in module {}", module_path!());
    };
}

mkfn!{
    sha256sig0_introspect!();
    # [doc = " Implements the Sigma0 transformation function as used in the SHA2-256 hash function \\[49\\]"] # [doc = " (Section 4.1.2)."] # [doc = ""] # [doc = " This instruction is supported for both RV32 and RV64 base architectures. For RV32, the"] # [doc = " entire XLEN source register is operated on. For RV64, the low 32 bits of the source"] # [doc = " register are operated on, and the result sign extended to XLEN bits. Though named for"] # [doc = " SHA2-256, the instruction works for both the SHA2-224 and SHA2-256 parameterisations as"] # [doc = " described in \\[49\\]. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.27"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha256sig0))] # [inline] pub fn sha256sig0 (rs1 : u32) -> u32 { unsafe { _sha256sig0 (rs1 as i32) as u32 } }
}

macro_rules! sha256sig1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha256sig1 in module {}", module_path!());
    };
}

mkfn!{
    sha256sig1_introspect!();
    # [doc = " Implements the Sigma1 transformation function as used in the SHA2-256 hash function \\[49\\]"] # [doc = " (Section 4.1.2)."] # [doc = ""] # [doc = " This instruction is supported for both RV32 and RV64 base architectures. For RV32, the"] # [doc = " entire XLEN source register is operated on. For RV64, the low 32 bits of the source"] # [doc = " register are operated on, and the result sign extended to XLEN bits. Though named for"] # [doc = " SHA2-256, the instruction works for both the SHA2-224 and SHA2-256 parameterisations as"] # [doc = " described in \\[49\\]. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.28"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha256sig1))] # [inline] pub fn sha256sig1 (rs1 : u32) -> u32 { unsafe { _sha256sig1 (rs1 as i32) as u32 } }
}

macro_rules! sha256sum0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha256sum0 in module {}", module_path!());
    };
}

mkfn!{
    sha256sum0_introspect!();
    # [doc = " Implements the Sum0 transformation function as used in the SHA2-256 hash function \\[49\\]"] # [doc = " (Section 4.1.2)."] # [doc = ""] # [doc = " This instruction is supported for both RV32 and RV64 base architectures. For RV32, the"] # [doc = " entire XLEN source register is operated on. For RV64, the low 32 bits of the source"] # [doc = " register are operated on, and the result sign extended to XLEN bits. Though named for"] # [doc = " SHA2-256, the instruction works for both the SHA2-224 and SHA2-256 parameterisations as"] # [doc = " described in \\[49\\]. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.29"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha256sum0))] # [inline] pub fn sha256sum0 (rs1 : u32) -> u32 { unsafe { _sha256sum0 (rs1 as i32) as u32 } }
}

macro_rules! sha256sum1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha256sum1 in module {}", module_path!());
    };
}

mkfn!{
    sha256sum1_introspect!();
    # [doc = " Implements the Sum1 transformation function as used in the SHA2-256 hash function \\[49\\]"] # [doc = " (Section 4.1.2)."] # [doc = ""] # [doc = " This instruction is supported for both RV32 and RV64 base architectures. For RV32, the"] # [doc = " entire XLEN source register is operated on. For RV64, the low 32 bits of the source"] # [doc = " register are operated on, and the result sign extended to XLEN bits. Though named for"] # [doc = " SHA2-256, the instruction works for both the SHA2-224 and SHA2-256 parameterisations as"] # [doc = " described in \\[49\\]. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.30"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha256sum1))] # [inline] pub fn sha256sum1 (rs1 : u32) -> u32 { unsafe { _sha256sum1 (rs1 as i32) as u32 } }
}

macro_rules! sm4ed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sm4ed in module {}", module_path!());
    };
}

mkfn!{
    sm4ed_introspect!();
    # [doc = " Accelerates the block encrypt/decrypt operation of the SM4 block cipher \\[5, 31\\]."] # [doc = ""] # [doc = " Implements a T-tables in hardware style approach to accelerating the SM4 round function. A"] # [doc = " byte is extracted from rs2 based on bs, to which the SBox and linear layer transforms are"] # [doc = " applied, before the result is XOR’d with rs1 and written back to rd. This instruction"] # [doc = " exists on RV32 and RV64 base architectures. On RV64, the 32-bit result is sign extended to"] # [doc = " XLEN bits. This instruction must always be implemented such that its execution latency does"] # [doc = " not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.43"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `BS` parameter is expected to be a constant value and only the bottom 2 bits of `bs` are"] # [doc = " used."] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " Accelerates the round function `F` in the SM4 block cipher algorithm"] # [doc = ""] # [doc = " This instruction is included in extension `Zksed`. It's defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " SM4ED(x, a, BS) = x ⊕ T(ai)"] # [doc = " ... where"] # [doc = " ai = a.bytes[BS]"] # [doc = " T(ai) = L(τ(ai))"] # [doc = " bi = τ(ai) = SM4-S-Box(ai)"] # [doc = " ci = L(bi) = bi ⊕ (bi ≪ 2) ⊕ (bi ≪ 10) ⊕ (bi ≪ 18) ⊕ (bi ≪ 24)"] # [doc = " SM4ED = (ci ≪ (BS * 8)) ⊕ x"] # [doc = " ```"] # [doc = ""] # [doc = " where `⊕` represents 32-bit xor, and `≪ k` represents rotate left by `k` bits."] # [doc = " As is defined above, `T` is a combined transformation of non linear S-Box transform `τ`"] # [doc = " and linear layer transform `L`."] # [doc = ""] # [doc = " In the SM4 algorithm, the round function `F` is defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " F(x0, x1, x2, x3, rk) = x0 ⊕ T(x1 ⊕ x2 ⊕ x3 ⊕ rk)"] # [doc = " ... where"] # [doc = " T(A) = L(τ(A))"] # [doc = " B = τ(A) = (SM4-S-Box(a0), SM4-S-Box(a1), SM4-S-Box(a2), SM4-S-Box(a3))"] # [doc = " C = L(B) = B ⊕ (B ≪ 2) ⊕ (B ≪ 10) ⊕ (B ≪ 18) ⊕ (B ≪ 24)"] # [doc = " ```"] # [doc = ""] # [doc = " It can be implemented by `sm4ed` instruction like:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[cfg(any(target_arch = \"riscv32\", target_arch = \"riscv64\"))]"] # [doc = " # fn round_function(x0: u32, x1: u32, x2: u32, x3: u32, rk: u32) -> u32 {"] # [doc = " # #[cfg(target_arch = \"riscv32\")] use core::arch::riscv32::sm4ed;"] # [doc = " # #[cfg(target_arch = \"riscv64\")] use core::arch::riscv64::sm4ed;"] # [doc = " let a = x1 ^ x2 ^ x3 ^ rk;"] # [doc = " let c0 = sm4ed(x0, a, 0);"] # [doc = " let c1 = sm4ed(c0, a, 1); // c1 represents c[0..=1], etc."] # [doc = " let c2 = sm4ed(c1, a, 2);"] # [doc = " let c3 = sm4ed(c2, a, 3);"] # [doc = " return c3; // c3 represents c[0..=3]"] # [doc = " # }"] # [doc = " ```"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zksed")] # [rustc_legacy_const_generics (2)] # [cfg_attr (test , assert_instr (sm4ed , BS = 0))] # [inline] pub fn sm4ed < const BS : u8 > (rs1 : u32 , rs2 : u32) -> u32 { static_assert ! (BS < 4) ; unsafe { _sm4ed (rs1 as i32 , rs2 as i32 , BS as i32) as u32 } }
}

macro_rules! sm4ks_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sm4ks in module {}", module_path!());
    };
}

mkfn!{
    sm4ks_introspect!();
    # [doc = " Accelerates the Key Schedule operation of the SM4 block cipher \\[5, 31\\] with `bs=0`."] # [doc = ""] # [doc = " Implements a T-tables in hardware style approach to accelerating the SM4 Key Schedule. A"] # [doc = " byte is extracted from rs2 based on bs, to which the SBox and linear layer transforms are"] # [doc = " applied, before the result is XOR’d with rs1 and written back to rd. This instruction"] # [doc = " exists on RV32 and RV64 base architectures. On RV64, the 32-bit result is sign extended to"] # [doc = " XLEN bits. This instruction must always be implemented such that its execution latency does"] # [doc = " not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.44"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `BS` parameter is expected to be a constant value and only the bottom 2 bits of `bs` are"] # [doc = " used."] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " Accelerates the round function `F` in the SM4 block cipher algorithm"] # [doc = ""] # [doc = " This instruction is included in extension `Zksed`. It's defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " SM4ED(x, a, BS) = x ⊕ T(ai)"] # [doc = " ... where"] # [doc = " ai = a.bytes[BS]"] # [doc = " T(ai) = L(τ(ai))"] # [doc = " bi = τ(ai) = SM4-S-Box(ai)"] # [doc = " ci = L(bi) = bi ⊕ (bi ≪ 2) ⊕ (bi ≪ 10) ⊕ (bi ≪ 18) ⊕ (bi ≪ 24)"] # [doc = " SM4ED = (ci ≪ (BS * 8)) ⊕ x"] # [doc = " ```"] # [doc = ""] # [doc = " where `⊕` represents 32-bit xor, and `≪ k` represents rotate left by `k` bits."] # [doc = " As is defined above, `T` is a combined transformation of non linear S-Box transform `τ`"] # [doc = " and linear layer transform `L`."] # [doc = ""] # [doc = " In the SM4 algorithm, the round function `F` is defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " F(x0, x1, x2, x3, rk) = x0 ⊕ T(x1 ⊕ x2 ⊕ x3 ⊕ rk)"] # [doc = " ... where"] # [doc = " T(A) = L(τ(A))"] # [doc = " B = τ(A) = (SM4-S-Box(a0), SM4-S-Box(a1), SM4-S-Box(a2), SM4-S-Box(a3))"] # [doc = " C = L(B) = B ⊕ (B ≪ 2) ⊕ (B ≪ 10) ⊕ (B ≪ 18) ⊕ (B ≪ 24)"] # [doc = " ```"] # [doc = ""] # [doc = " It can be implemented by `sm4ed` instruction like:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[cfg(any(target_arch = \"riscv32\", target_arch = \"riscv64\"))]"] # [doc = " # fn round_function(x0: u32, x1: u32, x2: u32, x3: u32, rk: u32) -> u32 {"] # [doc = " # #[cfg(target_arch = \"riscv32\")] use core::arch::riscv32::sm4ed;"] # [doc = " # #[cfg(target_arch = \"riscv64\")] use core::arch::riscv64::sm4ed;"] # [doc = " let a = x1 ^ x2 ^ x3 ^ rk;"] # [doc = " let c0 = sm4ed(x0, a, 0);"] # [doc = " let c1 = sm4ed(c0, a, 1); // c1 represents c[0..=1], etc."] # [doc = " let c2 = sm4ed(c1, a, 2);"] # [doc = " let c3 = sm4ed(c2, a, 3);"] # [doc = " return c3; // c3 represents c[0..=3]"] # [doc = " # }"] # [doc = " ```"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zksed")] # [rustc_legacy_const_generics (2)] # [cfg_attr (test , assert_instr (sm4ks , BS = 0))] # [inline] pub fn sm4ks < const BS : u8 > (rs1 : u32 , rs2 : u32) -> u32 { static_assert ! (BS < 4) ; unsafe { _sm4ks (rs1 as i32 , rs2 as i32 , BS as i32) as u32 } }
}

macro_rules! sm3p0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sm3p0 in module {}", module_path!());
    };
}

mkfn!{
    sm3p0_introspect!();
    # [doc = " Implements the P0 transformation function as used in the SM3 hash function [4, 30]."] # [doc = ""] # [doc = " This instruction is supported for the RV32 and RV64 base architectures. It implements the"] # [doc = " P0 transform of the SM3 hash function [4, 30]. This instruction must always be implemented"] # [doc = " such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.41"] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " `P0` transformation function as is used in the SM3 hash algorithm"] # [doc = ""] # [doc = " This function is included in `Zksh` extension. It's defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " P0(X) = X ⊕ (X ≪ 9) ⊕ (X ≪ 17)"] # [doc = " ```"] # [doc = ""] # [doc = " where `⊕` represents 32-bit xor, and `≪ k` represents rotate left by `k` bits."] # [doc = ""] # [doc = " In the SM3 algorithm, the `P0` transformation is used as `E ← P0(TT2)` when the"] # [doc = " compression function `CF` uses the intermediate value `TT2` to calculate"] # [doc = " the variable `E` in one iteration for subsequent processes."] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zksh")] # [cfg_attr (test , assert_instr (sm3p0))] # [inline] pub fn sm3p0 (rs1 : u32) -> u32 { unsafe { _sm3p0 (rs1 as i32) as u32 } }
}

macro_rules! sm3p1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sm3p1 in module {}", module_path!());
    };
}

mkfn!{
    sm3p1_introspect!();
    # [doc = " Implements the P1 transformation function as used in the SM3 hash function [4, 30]."] # [doc = ""] # [doc = " This instruction is supported for the RV32 and RV64 base architectures. It implements the"] # [doc = " P1 transform of the SM3 hash function [4, 30]. This instruction must always be implemented"] # [doc = " such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.42"] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " `P1` transformation function as is used in the SM3 hash algorithm"] # [doc = ""] # [doc = " This function is included in `Zksh` extension. It's defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " P1(X) = X ⊕ (X ≪ 15) ⊕ (X ≪ 23)"] # [doc = " ```"] # [doc = ""] # [doc = " where `⊕` represents 32-bit xor, and `≪ k` represents rotate left by `k` bits."] # [doc = ""] # [doc = " In the SM3 algorithm, the `P1` transformation is used to expand message,"] # [doc = " where expanded word `Wj` can be generated from the previous words."] # [doc = " The whole process can be described as the following pseudocode:"] # [doc = ""] # [doc = " ```text"] # [doc = " FOR j=16 TO 67"] # [doc = "     Wj ← P1(Wj−16 ⊕ Wj−9 ⊕ (Wj−3 ≪ 15)) ⊕ (Wj−13 ≪ 7) ⊕ Wj−6"] # [doc = " ENDFOR"] # [doc = " ```"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zksh")] # [cfg_attr (test , assert_instr (sm3p1))] # [inline] pub fn sm3p1 (rs1 : u32) -> u32 { unsafe { _sm3p1 (rs1 as i32) as u32 } }
}