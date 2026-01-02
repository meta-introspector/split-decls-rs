mkmod!{left, { 
                getname!(left);
                getsrc!(left);
                getpath!(left);
                get_deps!(left);
                get_crates!(left);
                mkinclude!(left);
                 
            }}
mkmod!{right, { 
                getname!(right);
                getsrc!(right);
                getpath!(right);
                get_deps!(right);
                get_crates!(right);
                mkinclude!(right);
                 
            }}