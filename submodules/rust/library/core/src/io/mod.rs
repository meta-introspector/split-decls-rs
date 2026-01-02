mkmod!{borrowed_buf, { 
                getname!(borrowed_buf);
                getsrc!(borrowed_buf);
                getpath!(borrowed_buf);
                get_deps!(borrowed_buf);
                get_crates!(borrowed_buf);
                mkinclude!(borrowed_buf);
                 
            }}
mkuse!{# [unstable (feature = "core_io_borrowed_buf" , issue = "117693")] pub use self :: borrowed_buf :: { BorrowedBuf , BorrowedCursor } ;}