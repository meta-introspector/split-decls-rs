macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_sm2_scalar_selectznz {
    () => {
        deps!();
        # [doc = " The function fiat_sm2_scalar_selectznz is a multi-limb conditional select."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (if arg1 = 0 then arg2 else arg3)"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [doc = "   arg3: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_sm2_scalar_selectznz (mut out1 : & mut [u64 ; 4] , arg1 : fiat_sm2_scalar_u1 , arg2 : & [u64 ; 4] , arg3 : & [u64 ; 4]) { let mut x1 : u64 = 0 ; fiat_sm2_scalar_cmovznz_u64 (& mut x1 , arg1 , (* IndexConst (arg2) . index (0)) , (* IndexConst (arg3) . index (0))) ; let mut x2 : u64 = 0 ; fiat_sm2_scalar_cmovznz_u64 (& mut x2 , arg1 , (* IndexConst (arg2) . index (1)) , (* IndexConst (arg3) . index (1))) ; let mut x3 : u64 = 0 ; fiat_sm2_scalar_cmovznz_u64 (& mut x3 , arg1 , (* IndexConst (arg2) . index (2)) , (* IndexConst (arg3) . index (2))) ; let mut x4 : u64 = 0 ; fiat_sm2_scalar_cmovznz_u64 (& mut x4 , arg1 , (* IndexConst (arg2) . index (3)) , (* IndexConst (arg3) . index (3))) ; * IndexConst (& mut out1) . index_mut (0) = x1 ; * IndexConst (& mut out1) . index_mut (1) = x2 ; * IndexConst (& mut out1) . index_mut (2) = x3 ; * IndexConst (& mut out1) . index_mut (3) = x4 ; }
    };
}

fiat_sm2_scalar_selectznz!();