mkmod!{cfi, { 
                getname!(cfi);
                getsrc!(cfi);
                getpath!(cfi);
                get_deps!(cfi);
                get_crates!(cfi);
                mkinclude!(cfi);
                 
            }}
mkmod!{kcfi, { 
                getname!(kcfi);
                getsrc!(kcfi);
                getpath!(kcfi);
                get_deps!(kcfi);
                get_crates!(kcfi);
                mkinclude!(kcfi);
                 
            }}