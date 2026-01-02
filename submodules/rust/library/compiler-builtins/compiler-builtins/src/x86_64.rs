mkuse!{use core :: intrinsics ;}
mkitem!{intrinsics ! { # [unsafe (naked)] # [cfg (any (all (windows , target_env = "gnu") , target_os = "cygwin" , target_os = "uefi"))] pub unsafe extern "custom" fn ___chkstk_ms () { core :: arch :: naked_asm ! ("push   %rcx" , "push   %rax" , "cmp    $0x1000,%rax" , "lea    24(%rsp),%rcx" , "jb     1f" , "2:" , "sub    $0x1000,%rcx" , "test   %rcx,(%rcx)" , "sub    $0x1000,%rax" , "cmp    $0x1000,%rax" , "ja     2b" , "1:" , "sub    %rax,%rcx" , "test   %rcx,(%rcx)" , "pop    %rax" , "pop    %rcx" , "ret" , options (att_syntax)) ; } }}
mkmod!{_fltused, { 
                getname!(_fltused);
                getsrc!(_fltused);
                getpath!(_fltused);
                get_deps!(_fltused);
                get_crates!(_fltused);
                mkinclude!(_fltused);
                mkitem!{# [unsafe (no_mangle)] # [used] # [cfg (target_os = "uefi")] static _fltused : i32 = 0 ;} 
            }}