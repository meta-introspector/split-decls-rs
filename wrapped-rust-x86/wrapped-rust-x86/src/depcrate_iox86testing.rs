// Generated macro for x86testing (module)
macro_rules! Depcrate_iox86testing {
() => {
// Module: crate::io
// Provides: {"x86testing"}
// Dependencies: {}
# [cfg (all (test , feature = "vmtest"))] mod x86testing { use super :: * ; use x86test :: * ; # [x86test (ioport (0x0 , 0xaf))] fn check_outb () { unsafe { outb (0x0 , 0xaf) ; } } # [x86test (ioport (0x0 , 0xaf))] # [should_panic] fn check_outb_wrong_value () { unsafe { outb (0x0 , 0xff) ; } } # [x86test (ioport (0x1 , 0xad))] fn check_inb () { unsafe { kassert ! (inb (0x1) == 0xad , "`inb` instruction didn't read the correct value") ; } } # [x86test (ioport (0x2 , 0xad))] # [should_panic] fn check_inb_wrong_port () { unsafe { kassert ! (inb (0x1) == 0xad , "`inb` instruction didn't read the correct value") ; } } # [x86test (ioport (0x2 , 0x99))] fn check_outw () { unsafe { super :: outw (0x2 , 0x99) ; } } # [x86test (ioport (0x3 , 0xfefe))] fn check_inw () { unsafe { kassert ! (inw (0x3) == 0xfefe , "`inw` instruction didn't read the correct value") ; } } # [x86test (ioport (0x5 , 0xbeefaaaa))] fn check_outl () { unsafe { outl (0x5 , 0xbeefaaaa) ; } } # [x86test (ioport (0x4 , 0xdeadbeef))] fn check_inl () { unsafe { kassert ! (inl (0x4) == 0xdeadbeef , "`inl` instruction didn't read the correct value") ; } } }
};
}
