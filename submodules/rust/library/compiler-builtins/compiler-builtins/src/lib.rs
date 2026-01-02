mkitem!{# [cfg (test)] extern crate core ;}
mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{float, { 
                getname!(float);
                getsrc!(float);
                getpath!(float);
                get_deps!(float);
                get_crates!(float);
                mkinclude!(float);
                 
            }}
mkmod!{int, { 
                getname!(int);
                getsrc!(int);
                getpath!(int);
                get_deps!(int);
                get_crates!(int);
                mkinclude!(int);
                 
            }}
mkmod!{math, { 
                getname!(math);
                getsrc!(math);
                getpath!(math);
                get_deps!(math);
                get_crates!(math);
                mkinclude!(math);
                 
            }}
mkmod!{mem, { 
                getname!(mem);
                getsrc!(mem);
                getpath!(mem);
                get_deps!(mem);
                get_crates!(mem);
                mkinclude!(mem);
                 
            }}
mkuse!{use math :: libm_math :: support ;}
mkmod!{arm, { 
                getname!(arm);
                getsrc!(arm);
                getpath!(arm);
                get_deps!(arm);
                get_crates!(arm);
                mkinclude!(arm);
                 
            }}
mkmod!{aarch64, { 
                getname!(aarch64);
                getsrc!(aarch64);
                getpath!(aarch64);
                get_deps!(aarch64);
                get_crates!(aarch64);
                mkinclude!(aarch64);
                 
            }}
mkmod!{aarch64_linux, { 
                getname!(aarch64_linux);
                getsrc!(aarch64_linux);
                getpath!(aarch64_linux);
                get_deps!(aarch64_linux);
                get_crates!(aarch64_linux);
                mkinclude!(aarch64_linux);
                 
            }}
mkmod!{arm_linux, { 
                getname!(arm_linux);
                getsrc!(arm_linux);
                getpath!(arm_linux);
                get_deps!(arm_linux);
                get_crates!(arm_linux);
                mkinclude!(arm_linux);
                 
            }}
mkmod!{avr, { 
                getname!(avr);
                getsrc!(avr);
                getpath!(avr);
                get_deps!(avr);
                get_crates!(avr);
                mkinclude!(avr);
                 
            }}
mkmod!{hexagon, { 
                getname!(hexagon);
                getsrc!(hexagon);
                getpath!(hexagon);
                get_deps!(hexagon);
                get_crates!(hexagon);
                mkinclude!(hexagon);
                 
            }}
mkmod!{riscv, { 
                getname!(riscv);
                getsrc!(riscv);
                getpath!(riscv);
                get_deps!(riscv);
                get_crates!(riscv);
                mkinclude!(riscv);
                 
            }}
mkmod!{x86, { 
                getname!(x86);
                getsrc!(x86);
                getpath!(x86);
                get_deps!(x86);
                get_crates!(x86);
                mkinclude!(x86);
                 
            }}
mkmod!{x86_64, { 
                getname!(x86_64);
                getsrc!(x86_64);
                getpath!(x86_64);
                get_deps!(x86_64);
                get_crates!(x86_64);
                mkinclude!(x86_64);
                 
            }}
mkmod!{probestack, { 
                getname!(probestack);
                getsrc!(probestack);
                getpath!(probestack);
                get_deps!(probestack);
                get_crates!(probestack);
                mkinclude!(probestack);
                 
            }}