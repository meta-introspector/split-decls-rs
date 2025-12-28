macro_rules! deps {
    () => {
        X86!();
    };
}

macro_rules! macro_53 {
    () => {
        deps!();
        registers ! (X86 , { EAX = (0 , "eax") , ECX = (1 , "ecx") , EDX = (2 , "edx") , EBX = (3 , "ebx") , ESP = (4 , "esp") , EBP = (5 , "ebp") , ESI = (6 , "esi") , EDI = (7 , "edi") , RA = (8 , "RA") , ST0 = (11 , "st0") , ST1 = (12 , "st1") , ST2 = (13 , "st2") , ST3 = (14 , "st3") , ST4 = (15 , "st4") , ST5 = (16 , "st5") , ST6 = (17 , "st6") , ST7 = (18 , "st7") , XMM0 = (21 , "xmm0") , XMM1 = (22 , "xmm1") , XMM2 = (23 , "xmm2") , XMM3 = (24 , "xmm3") , XMM4 = (25 , "xmm4") , XMM5 = (26 , "xmm5") , XMM6 = (27 , "xmm6") , XMM7 = (28 , "xmm7") , MM0 = (29 , "mm0") , MM1 = (30 , "mm1") , MM2 = (31 , "mm2") , MM3 = (32 , "mm3") , MM4 = (33 , "mm4") , MM5 = (34 , "mm5") , MM6 = (35 , "mm6") , MM7 = (36 , "mm7") , MXCSR = (39 , "mxcsr") , ES = (40 , "es") , CS = (41 , "cs") , SS = (42 , "ss") , DS = (43 , "ds") , FS = (44 , "fs") , GS = (45 , "gs") , TR = (48 , "tr") , LDTR = (49 , "ldtr") , FS_BASE = (93 , "fs.base") , GS_BASE = (94 , "gs.base") , }) ;
    };
}

macro_53!();